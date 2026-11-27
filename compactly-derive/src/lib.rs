use synstructure::decl_derive;

mod v1;
mod v2;

decl_derive!(
    [Encode, attributes(compactly)] =>
    /// Derive the stable `v1` [`Encode`](https://docs.rs/compactly/latest/compactly/v1/trait.Encode.html) trait.
    ///
    /// This is an alias for [`EncodeV1`](derive.EncodeV1.html); it produces the
    /// binary-stable `v1` format. The generated `Context` stores one field per
    /// struct or enum field, so each field's probability model adapts
    /// independently.
    ///
    /// Annotate fields with `#[compactly(Strategy)]` to pick an
    /// [encoding strategy](https://docs.rs/compactly/latest/compactly/index.html#encoding-strategies), e.g.
    /// `#[compactly(Small)]` or `#[compactly(LowCardinality)]`.
    v1::derive_compactly
);

decl_derive!(
    [EncodeV1, attributes(compactly)] =>
    /// Derive the stable `v1` [`Encode`](https://docs.rs/compactly/latest/compactly/v1/trait.Encode.html) trait.
    ///
    /// Same as [`Encode`](derive.Encode.html), which derives `v1` — note that
    /// the bare `Encode` from *this* crate is `v1`, unlike
    /// [`compactly::Encode`](https://docs.rs/compactly/latest/compactly/derive.Encode.html),
    /// the re-export most users import, which is `v2`. Reach for this explicit
    /// spelling when both `v1` and `v2` are in play and you want to be
    /// unambiguous. Fields may carry `#[compactly(Strategy)]` attributes to
    /// select an encoding strategy.
    v1::derive_compactly
);

decl_derive!(
    [EncodeV2, attributes(compactly)] =>
    /// Derive the default `v2` [`Encode`](https://docs.rs/compactly/latest/compactly/v2/trait.Encode.html) trait.
    ///
    /// `v2` is the format re-exported as `compactly::{encode, decode, Encode}`.
    /// The generated `Context` stores one field per struct or enum field, so
    /// each field's probability model adapts independently. Fields may carry
    /// `#[compactly(Strategy)]` attributes to select an encoding strategy.
    ///
    /// `Encode<S>` bundles the sync `encode`/`decode` and the async-decode
    /// members (`MAX_BYTES`, `decode_awaiting`) that `v2::decode_stream` needs
    /// into one trait with no opt-out, so this single derive always produces
    /// both — there is no separate "async" derive to reach for.
    v2::derive_compactly
);

/// Parsed contents of the `#[compactly(...)]` attributes on a field or container.
pub(crate) struct CompactlyAttrs {
    /// Encoding-strategy types (e.g. `Small`, `LowCardinality`, `Mapping<K, V>`).
    /// At most one is meaningful per field; a container newtype may carry several.
    pub strategies: Vec<syn::Type>,
    /// Whether the `allow_string` flag was present, opting this field out of the
    /// `LowCardinality<String>` deprecation warning emitted by the v2 derive.
    pub allow_string: bool,
}

impl CompactlyAttrs {
    /// The single field-level encoding strategy, if any. A field may carry at
    /// most one strategy; more than one is meaningless there, so panic — only a
    /// container newtype legitimately carries several, and it reads `strategies`
    /// directly. `binding` is included in the panic for diagnostics. Kept here,
    /// rather than duplicated in the v1 and v2 derives, so the check and its
    /// message can't drift apart.
    pub fn single_strategy(&self, binding: &impl std::fmt::Debug) -> Option<syn::Type> {
        match self.strategies.as_slice() {
            [] => None,
            [s] => Some(s.clone()),
            _ => panic!("Cannot support multiple encoding strategies: {binding:?}"),
        }
    }
}

/// The set of bare-ident flags `#[compactly(...)]` understands. Anything else in
/// snake_case is rejected as a typo rather than mistaken for a strategy type.
const KNOWN_FLAGS: &[&str] = &["allow_string"];

/// Parse every `#[compactly(...)]` attribute in `attrs`, collecting the encoding
/// strategies and recognizing bare flag idents (currently just `allow_string`).
///
/// The contents are a comma-separated list, so `#[compactly(LowCardinality,
/// allow_string)]` yields the `LowCardinality` strategy with `allow_string` set.
/// The flag is shared by both derives so a type deriving `EncodeV1` and
/// `EncodeV2` can carry it without the v1 derive choking on it.
pub(crate) fn parse_compactly_attrs(attrs: &[syn::Attribute]) -> CompactlyAttrs {
    use syn::punctuated::Punctuated;
    let mut strategies = Vec::new();
    let mut allow_string = false;
    for a in attrs {
        if !a.path().is_ident("compactly") {
            continue;
        }
        let items = a
            .parse_args_with(Punctuated::<syn::Type, syn::Token![,]>::parse_terminated)
            .expect("Unrecognized compactly attribute");
        for item in items {
            match bare_ident(&item) {
                Some(ident) if *ident == "allow_string" => allow_string = true,
                // A bare snake_case ident that isn't a known flag is almost
                // certainly a misspelled flag: encoding-strategy types are
                // UpperCamelCase, so a lowercase-initial ident was never meant as
                // one. Reject it clearly instead of pushing it into `strategies`,
                // where it would later blow up with an opaque BindingInfo dump.
                Some(ident)
                    if starts_lowercase(ident)
                        && !KNOWN_FLAGS.contains(&ident.to_string().as_str()) =>
                {
                    panic!(
                        "unknown compactly flag `{ident}`; expected an encoding strategy or one of {KNOWN_FLAGS:?}"
                    )
                }
                _ => {
                    let mut strategy = item;
                    qualify_builtin_strategies(&mut strategy);
                    strategies.push(strategy);
                }
            }
        }
    }
    CompactlyAttrs {
        strategies,
        allow_string,
    }
}

/// The strategy types `compactly` defines, which a `#[compactly(...)]`
/// attribute may name without importing them.
const BUILTIN_STRATEGIES: &[&str] = &[
    "Small",
    "LowCardinality",
    "Decimal",
    "Compressible",
    "Incompressible",
    "Mapping",
    "Normal",
    "Sorted",
    "Values",
];

/// Spell each built-in strategy that `ty` names bare as a path from the crate
/// root, so that `Mapping<Small, Normal>` becomes
/// `compactly::Mapping<compactly::Small, compactly::Normal>`.
///
/// This is what lets `#[compactly(Small)]` work without a `use`, without the
/// generated code importing those names. An import would bring them into scope
/// for the whole expansion, which holds the user's field types and the type's
/// own name as well, so a type of the user's called `Values` or `Normal` would
/// stop meaning itself there. Any other name, such as a strategy of the user's
/// own, is left to resolve where the attribute was written.
fn qualify_builtin_strategies(ty: &mut syn::Type) {
    let syn::Type::Path(type_path) = ty else {
        return;
    };
    let path = &mut type_path.path;
    if type_path.qself.is_none() && path.leading_colon.is_none() && path.segments.len() == 1 {
        let name = &path.segments[0].ident;
        if BUILTIN_STRATEGIES.iter().any(|builtin| name == builtin) {
            let krate = syn::Ident::new("compactly", name.span());
            path.segments.insert(0, krate.into());
        }
    }
    for segment in &mut path.segments {
        if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
            for arg in &mut args.args {
                if let syn::GenericArgument::Type(inner) = arg {
                    qualify_builtin_strategies(inner);
                }
            }
        }
    }
}

/// Names a field's binding must not take, because the generated code uses them
/// where the bindings are in scope: `discriminant` is a field of the generated
/// context, and `writer` and `ctx` are parameters of the `encode` method whose
/// `match` binds every field. A field with one of these names is bound as
/// `writer_0` and so on instead, which nothing outside the expansion sees.
pub(crate) const RESERVED_BINDING_NAMES: &[&str] = &["discriminant", "writer", "ctx"];

/// A name for a type parameter of the generated methods that none of the type's
/// own generic parameters uses: `preferred` itself when it is free, and
/// otherwise the first free one of `preferred0`, `preferred1`, ….
///
/// A method's generics share a scope with its impl's, so a type with a
/// parameter `E` could not take a generated `fn encode<E: EntropyCoder>`.
pub(crate) fn fresh_type_param(generics: &syn::Generics, preferred: &str) -> proc_macro2::Ident {
    let taken = generics
        .params
        .iter()
        .filter_map(|param| match param {
            syn::GenericParam::Type(t) => Some(t.ident.clone()),
            syn::GenericParam::Const(c) => Some(c.ident.clone()),
            syn::GenericParam::Lifetime(_) => None,
        })
        .collect::<std::collections::BTreeSet<_>>();
    let ident = proc_macro2::Ident::new(preferred, proc_macro2::Span::call_site());
    if taken.contains(&ident) {
        get_unique_name(&taken, preferred, u32::MAX)
    } else {
        ident
    }
}

/// If `ty` is a bare single-segment identifier (no path qualifier, no generic
/// arguments) — as opposed to something like `Mapping<K, V>` — return it.
fn bare_ident(ty: &syn::Type) -> Option<&syn::Ident> {
    match ty {
        syn::Type::Path(p) if p.qself.is_none() => p.path.get_ident(),
        _ => None,
    }
}

/// Whether `ident`'s first character is lowercase (Rust's convention for values
/// and flags, as opposed to the UpperCamelCase of strategy types).
fn starts_lowercase(ident: &syn::Ident) -> bool {
    ident
        .to_string()
        .chars()
        .next()
        .is_some_and(char::is_lowercase)
}

pub(crate) fn get_unique_name(
    bound_names: &std::collections::BTreeSet<proc_macro2::Ident>,
    prefix: &str,
    tries: u32,
) -> proc_macro2::Ident {
    for idx in 0..tries {
        let ident =
            proc_macro2::Ident::new(&format!("{prefix}{idx}"), proc_macro2::Span::call_site());
        if !bound_names.contains(&ident) {
            return ident;
        }
    }
    panic!(
        "compactly does not currently support types with more than {tries} identical field names"
    );
}

#[test]
fn builtin_strategies_are_spelled_from_the_crate_root() {
    let attrs: Vec<syn::Attribute> = vec![syn::parse_quote! {
        #[compactly(Mapping<Small, Values<Sorted>>, my::Small, Strategy)]
    }];
    let expected: [syn::Type; 3] = [
        syn::parse_quote!(compactly::Mapping<compactly::Small, compactly::Values<compactly::Sorted>>),
        // A path, or a name that is not compactly's, is the user's to resolve.
        syn::parse_quote!(my::Small),
        syn::parse_quote!(Strategy),
    ];
    assert_eq!(parse_compactly_attrs(&attrs).strategies, expected);
}
