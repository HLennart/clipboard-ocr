fn main() {
    // println!("cargo:rerun-if-changed=tessdata");
    let out_dir = std::env::var("OUT_DIR").unwrap();

    std::fs::copy("tessdata/eng.traineddata", format!("{}/eng.traineddata", out_dir)).unwrap();
}