#[feature(struct_variant, macro_rules)];
#[crate_id="github.com/sfackler/rust-openssl#openssl:0.0"];
#[crate_type="lib"];
#[doc(html_root_url="http://www.rust-ci.org/sfackler/rust-openssl/doc")];

extern mod extra;
extern mod sync;

pub mod ssl;
pub mod crypto;