use synstructure::decl_derive;

mod v1;
mod v2;

decl_derive!([Encode, attributes(compactly)] => v1::derive_compactly);

decl_derive!([EncodeV1, attributes(compactly)] => v1::derive_compactly);

decl_derive!([EncodeV2, attributes(compactly)] => v2::derive_compactly);

/// Parsed contents of the `#[compactly(...)]` attributes on a field or container.
pub(crate) struct CompactlyAttrs {
    /// Encoding-strategy types (e.g. `Small`, `LowCardinality`, `Mapping<K, V>`).
    /// At most one is meaningful per field; a container newtype may carry several.
    pub strategies: Vec<syn::Type>,
    /// Whether the `allow_string` flag was present, opting this field out of the
    /// `LowCardinality<String>` deprecation warning emitted by the v2 derive.
    pub allow_string: bool,
}

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
            if matches!(&item, syn::Type::Path(p) if p.qself.is_none() && p.path.is_ident("allow_string"))
            {
                allow_string = true;
            } else {
                strategies.push(item);
            }
        }
    }
    CompactlyAttrs {
        strategies,
        allow_string,
    }
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
