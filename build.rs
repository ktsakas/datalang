use std::fs;
use std::path::Path;

fn main() {
    let text_definitions_dir = "dirctionary_tests/text_definitions";
    let macro_definitions_dir = "dirctionary_tests/macro_definitions";
    
    // Create the macro_definitions directory if it doesn't exist
    fs::create_dir_all(macro_definitions_dir).unwrap();
    
    // Read all .txt files from text_definitions
    let text_dir = Path::new(text_definitions_dir);
    if text_dir.exists() && text_dir.is_dir() {
        for entry in fs::read_dir(text_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let content = fs::read_to_string(&path).unwrap();
                
                // Generate the .rs file with datalang! macro wrapper
                // Add leading tab to each line of content
                let indented_content = content
                    .lines()
                    .map(|line| format!("\t{}", line))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                let rust_content = format!(
                    "use datalang::datalang;\n\ndatalang! {{\n{}\n}}\n",
                    indented_content
                );
                
                let output_path = Path::new(macro_definitions_dir).join(format!("{}.rs", file_stem));
                fs::write(output_path, rust_content).unwrap();
                
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
    
    // Rerun if the text_definitions directory changes
    println!("cargo:rerun-if-changed={}", text_definitions_dir);
    println!("cargo:rerun-if-changed=build.rs");
}