use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// Directory containing input icon files
const ICONS_DIR: &str = "icons";
/// Directory to save the generated `.rc` files
const OUTPUT_DIR: &str = "output";

/// Generates a `.rc` file for the specified icon
fn generate_rc_file(icon_path: &str, output_path: &str) -> io::Result<()> {
    let icon_name = Path::new(icon_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("icon");

    let rc_content = format!(
        "id ICON \"{}\"\n",
        icon_path
    );

    let mut output_file = File::create(output_path)?;
    output_file.write_all(rc_content.as_bytes())?;
    println!("Generated .rc file: {}", output_path);
    Ok(())
}

fn main() -> io::Result<()> {
    // Ensure output directory exists
    fs::create_dir_all(OUTPUT_DIR)?;

    // Iterate over all icon files in the icons directory
    for entry in fs::read_dir(ICONS_DIR)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "ico" {
                    // Generate a corresponding .rc file
                    let output_path = format!(
                        "{}/{}.rc",
                        OUTPUT_DIR,
                        path.file_stem().unwrap().to_str().unwrap()
                    );
                    generate_rc_file(path.to_str().unwrap(), &output_path)?;
                }
            }
        }
    }

    println!("All .rc files generated successfully.");
    Ok(())
}
