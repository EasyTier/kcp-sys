use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=kcp");
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let fulldir = Path::new(&dir).join("kcp");

    let mut config = cc::Build::new();
    config.include(fulldir.clone());
    config.file(fulldir.join("ikcp.c"));
    config.opt_level(3);
    config.warnings(false);
    config.compile("libkcp.a");
    println!("cargo:rustc-link-search=native={}", fulldir.display());

    println!("cargo:rerun-if-changed=kcp/ikcp.h");
    println!("cargo:rerun-if-changed=kcp/ikcp.c");
    println!("cargo:rerun-if-changed=wrapper.h");

    // Get kcp submodule commit ID
    let kcp_commit = get_kcp_commit_id(&fulldir);
    let src_dir = Path::new(&dir).join("src");
    let binding_file = src_dir.join(format!("binding_{}.rs", kcp_commit));
    
    println!("cargo:rerun-if-changed={}", binding_file.display());

    // If pregenerated bindings file exists, use it
    if binding_file.exists() {
        println!("cargo:rustc-cfg=use_pregenerated_bindings");
        println!("Using pregenerated bindings: {}", binding_file.display());
        
        // Copy pregenerated bindings file to OUT_DIR
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let target_bindings = out_path.join("bindings.rs");
        std::fs::copy(&binding_file, &target_bindings)
            .expect("Failed to copy pregenerated bindings");
    } else {
        // Check if bindgen feature is enabled
        #[cfg(feature = "bindgen")]
        {
            println!("cargo:warning=Pregenerated bindings not found for commit {}, generating with bindgen", kcp_commit);
            println!("cargo:warning=Consider committing {} to avoid bindgen dependency", binding_file.display());
            generate_bindings(&dir, &binding_file);
        }
        
        #[cfg(not(feature = "bindgen"))]
        {
            panic!(
                "Pregenerated bindings not found for kcp commit: {}\n\
                 Expected file: {}\n\
                 \n\
                 To fix this issue, you have two options:\n\
                 1. Enable bindgen feature: cargo build --features bindgen\n\
                 2. Ask the maintainer to generate bindings for this kcp version\n\
                 \n\
                 If you are the maintainer, run: cargo build --features bindgen\n\
                 Then commit the generated file: {}",
                kcp_commit,
                binding_file.display(),
                binding_file.display()
            );
        }
    }
}

fn get_kcp_commit_id(kcp_dir: &Path) -> String {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(kcp_dir)
        .output()
        .expect("Failed to get kcp commit ID");

    if !output.status.success() {
        panic!("Failed to get kcp commit ID: {}", String::from_utf8_lossy(&output.stderr));
    }

    String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 in git output")
        .trim()
        .to_string()
}

#[cfg(feature = "bindgen")]
fn generate_bindings(_manifest_dir: &str, binding_file: &Path) {
    let extra_header_path = std::env::var("KCP_SYS_EXTRA_HEADER_PATH").unwrap_or_default();
    let extra_header_paths = extra_header_path
        .split(":")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(extra_header_paths.iter().map(|p| format!("-I{}", p)))
        .allowlist_function("ikcp_.*")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_bindings = out_path.join("bindings.rs");
    
    // Write to OUT_DIR (for build-time use)
    bindings
        .write_to_file(&target_bindings)
        .expect("Couldn't write bindings to OUT_DIR!");
    
    // Also write to src directory (for committing)
    bindings
        .write_to_file(binding_file)
        .expect("Couldn't write bindings to src directory!");
        
    println!("Generated bindings for current kcp commit: {}", binding_file.display());
    println!("Please commit this file to avoid bindgen dependency for other users");
}
