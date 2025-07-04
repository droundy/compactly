use synstructure::decl_derive;

mod ans;
mod v1;

decl_derive!([Encode, attributes(compactly)] => v1::derive_compactly);

decl_derive!([EncodeV1, attributes(compactly)] => v1::derive_compactly);

decl_derive!([EncodeAns, attributes(compactly)] => ans::derive_compactly);
