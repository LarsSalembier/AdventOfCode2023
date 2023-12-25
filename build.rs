use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:warning=Executing build script...");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR");
    if manifest_dir.is_err() {
        println!("cargo:warning=Could not find manifest directory. Make sure the CARGO_MANIFEST_DIR environment variable is set.");
        panic!("Could not find manifest directory. Make sure the CARGO_MANIFEST_DIR environment variable is set.");
    }

    let out_dir = env::var("OUT_DIR");
    if out_dir.is_err() {
        println!("cargo:warning=Could not find output directory. Make sure the OUT_DIR environment variable is set.");
        panic!("Could not find output directory. Make sure the OUT_DIR environment variable is set.");
    }

    let input_dir = Path::new(&manifest_dir.unwrap()).join("inputs");
    let output_dir = Path::new(&out_dir.unwrap()).join("inputs");

    let result = fs::create_dir_all(&output_dir);
    if result.is_err() {
        let error = result.unwrap_err();
        println!("cargo:warning=Could not create output directory: {:?}. {:?}", output_dir, error);
        panic!("Could not create output directory: {:?}. {:?}", output_dir, error);
    }

    if input_dir.exists() && input_dir.is_dir() {
        let entries = fs::read_dir(&input_dir);
        if entries.is_err() {
            let error = entries.unwrap_err();
            println!("cargo:warning=Could not read input directory: {:?}. {:?}", input_dir, error);
            panic!("Could not read input directory: {:?}. {:?}", input_dir, error);
        }

        for entry in entries.unwrap() {
            let entry = entry;
            if entry.is_err() {
                let error = entry.unwrap_err();
                println!("cargo:warning=Could not read entry in input directory: {:?}. {:?}", input_dir, error);
                panic!("Could not read entry in input directory: {:?}. {:?}", input_dir, error);
            }
            let entry_ = entry.unwrap();
            let entry_name = entry_.file_name();
            let entry_path = entry_.path();
            let destination_path = output_dir.join(entry_name);

            let result = fs::copy(&entry_path, &destination_path);
            if result.is_err() {
                let error = result.unwrap_err();
                println!("cargo:warning=Could not copy input file: {:?}. {:?}", &entry_path, error);
                panic!("Could not copy input file: {:?}. {:?}", entry_path, error);
            }
        }
    } else {
        println!("cargo:warning=Input directory not found");
    }
}