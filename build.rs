fn main() {
    // Tell cargo to tell rustc to link the system eyelink shared libraries.
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=eyelink_core_graphics");
    println!("cargo:rustc-link-lib=eyelink_core");
}
