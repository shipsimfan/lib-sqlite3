const SOURCE: &str = "sqlite3.c";
const LIBRARY_NAME: &str = "sqlite3";

fn main() {
    // Inform rust to only re-run if the source file changes
    println!("cargo:rerun-if-changed={}", SOURCE);

    // Compile the source file
    cc::Build::new().file(SOURCE).compile(LIBRARY_NAME);
}
