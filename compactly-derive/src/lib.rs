use synstructure::decl_derive;

mod v0;
mod v1;

decl_derive!([Encode, attributes(compactly)] => v1::derive_compactly);

decl_derive!([EncodeV0, attributes(compactly)] => v0::derive_compactly);

decl_derive!([EncodeV1, attributes(compactly)] => v1::derive_compactly);
