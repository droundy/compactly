use synstructure::decl_derive;

mod v1;
mod v2;

decl_derive!([Encode, attributes(compactly)] => v1::derive_compactly);

decl_derive!([EncodeV1, attributes(compactly)] => v1::derive_compactly);

decl_derive!([EncodeV2, attributes(compactly)] => v2::derive_compactly);
