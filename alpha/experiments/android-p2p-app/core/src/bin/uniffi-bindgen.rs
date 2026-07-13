//! Standalone bindgen entry point. UniFFI's recommended pattern: build the library,
//! then run this binary to generate the foreign-language bindings (Kotlin) from it:
//!
//! ```sh
//! cargo build
//! cargo run --bin uniffi-bindgen -- generate \
//!     --library target/debug/libgroup_core.so \
//!     --language kotlin --out-dir ../android/app/src/main/java
//! ```
fn main() {
    uniffi::uniffi_bindgen_main()
}
