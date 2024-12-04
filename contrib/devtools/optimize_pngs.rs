use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Directory containing PNG images to optimize
const PNGS_DIR: &str = "assets/pngs";
/// Directory to store optimized PNG images
const OUTPUT_DIR: &str = "assets/optimized";

/// Optimizes a single PNG file using `oxipng`
fn optimize_png(file_path: &Path, output_dir: &Path) -> std::io::Result<()> {
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    let output_path = output_dir.join(file_path.file_name().unwrap());

    // Use oxipng to optimize the PNG
    let status = Command::new("oxipng")
        .arg("--opt")
        .arg("max")
        .arg("--out")
        .arg(output_path.to_str().unwrap())
        .arg(file_path.to_str().unwrap())
        .status()?;

    if !status.success() {
        eprintln!(
            "Failed to optimize {}",
            file_path.to_str().unwrap_or("unknown file")
        );
    } else {
        println!(
            "Optimized {} -> {}",
            file_path.to_str().unwrap_or("unknown file"),
            output_path.to_str().unwrap_or("unknown output")
        );
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let png_dir = Path::new(PNGS_DIR);
    let output_dir = Path::new(OUTPUT_DIR);

    if !png_dir.exists() {
        eprintln!("PNG directory {} does not exist.", PNGS_DIR);
        return Ok(());
    }

    // Iterate through all PNG files in the directory
    for entry in fs::read_dir(png_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "png" {
            optimize_png(&path, output_dir)?;
        }
    }

    println!("All PNG files optimized successfully.");
    Ok(())
}
