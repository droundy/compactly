use synstructure::decl_derive;

mod v1;
mod v2;

decl_derive!([Encode, attributes(compactly)] => v1::derive_compactly);

decl_derive!([EncodeV1, attributes(compactly)] => v1::derive_compactly);

decl_derive!([EncodeV2, attributes(compactly)] => v2::derive_compactly);

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
