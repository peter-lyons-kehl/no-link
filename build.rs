fn main() {
    // The following did NOT break linking.
    //
    //println!("cargo:rustc-link-arg=-U_no_build_crate_explicitly_breaks_build");

    // See https://doc.rust-lang.org/nightly/cargo/reference/build-scripts.html#outputs-of-the-build-script
    println!("cargo:rustc-link-lib=no_build_crate_explicitly_breaks_build");
}
