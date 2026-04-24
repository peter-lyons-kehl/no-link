fn main() {
    // The following did NOT break linking.
    //
    //println!("cargo:rustc-link-arg=-U_no_build_crate_explicitly_breaks_build");

    println!("cargo:rustc-link-lib=no_build_crate_explicitly_breaks_build");
}
