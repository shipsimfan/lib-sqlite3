use std::{env, path::Path, process::Command};

const SOURCE: &str = "sqlite3.c";
const OBJECT: &str = "sqlite3.o";
const OUTPUT: &str = "libsqlite3.a";

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let obj_path = Path::new(&out_dir).join(OBJECT);
    let library_path = Path::new(&out_dir).join(OUTPUT);

    println!("cargo::rerun-if-changed={}", SOURCE);

    Command::new("cc")
        .args([SOURCE, "-c", "-o"])
        .arg(&obj_path)
        .output()
        .expect("Failed to compile sqlite");

    Command::new("ar")
        .arg("rcs")
        .args([library_path, obj_path])
        .output()
        .expect("Failed to archive sqlite");

    println!("cargo::rustc-link-lib=sqlite3");
}
