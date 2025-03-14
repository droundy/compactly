use synstructure::decl_derive;

mod v0;

decl_derive!([Encode, attributes(compactly)] => v0::derive_compactly);

decl_derive!([EncodeV0, attributes(compactly)] => v0::derive_compactly);
