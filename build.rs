use std::process::Command;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Compile GResource
    println!("cargo:rerun-if-changed=resources.gresource.xml");
    println!("cargo:rerun-if-changed=assets");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    
    // Determine the target profile directory (where the binary is placed)
    // OUT_DIR is target/{profile}/build/{crate}-{hash}/out/
    let target_dir = PathBuf::from(&out_dir)
        .ancestors()
        .nth(3)
        .expect("Failed to determine target directory")
        .to_path_buf();

    // Compile the GResource file
    let status = Command::new("glib-compile-resources")
        .arg("resources.gresource.xml")
        .arg("--target")
        .arg(format!("{}/resources.gresource", out_dir))
        .arg("--sourcedir")
        .arg(".")
        .status()
        .expect("Failed to execute glib-compile-resources. Make sure it is installed.");

    if !status.success() {
        panic!("glib-compile-resources failed");
    }

    // Compile .po translation files to .mo
    // Place them in target/{profile}/locale/{lang}/LC_MESSAGES/ next to the binary
    println!("cargo:rerun-if-changed=po");

    let po_dir = Path::new("po");
    if po_dir.exists() {
        for entry in fs::read_dir(po_dir).expect("Failed to read po/ directory") {
            let entry = entry.expect("Failed to read po/ entry");
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("po") {
                let lang = path.file_stem().unwrap().to_str().unwrap();
                let mo_dir = target_dir.join(format!("locale/{}/LC_MESSAGES", lang));
                let mo_file = mo_dir.join("xrayhexgenerator.mo");

                fs::create_dir_all(&mo_dir)
                    .unwrap_or_else(|e| panic!("Failed to create locale dir {}: {}", mo_dir.display(), e));

                let status = Command::new("msgfmt")
                    .arg(path.to_str().unwrap())
                    .arg("-o")
                    .arg(mo_file.to_str().unwrap())
                    .status()
                    .expect("Failed to execute msgfmt. Make sure gettext tools are installed.");

                if !status.success() {
                    panic!("msgfmt failed for {}", path.display());
                }
            }
        }
    }
}
