fn main() {
    // we don't need to rebuild for anything else
    println!("cargo:rerun-if-changed=build.rs");
}
