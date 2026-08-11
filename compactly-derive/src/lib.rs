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
    v2::derive_compactly
);

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
