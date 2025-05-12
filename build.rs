use std::{env, path::PathBuf};

fn main() {
    // Link to symengine
    println!("cargo:rustc-link-lib=symengine");
    
    // Ensure that symengine directory is used if specified
    let symengine_dir = env::var("SYMENGINE_DIR").ok();
    if let Some(dir) = &symengine_dir {
        // Add include path
        println!("cargo:rustc-link-search={}/lib", dir);
        
        // Set clang arguments for bindgen
        let mut bindgen_args = Vec::new();
        bindgen_args.push(format!("-I{}/include", dir));
        
        // Add GMP and MPFR include paths if specified
        if let Ok(gmp_dir) = env::var("GMP_DIR") {
            bindgen_args.push(format!("-I{}/include", gmp_dir));
            println!("cargo:rustc-link-search={}/lib", gmp_dir);
        }
        
        if let Ok(mpfr_dir) = env::var("MPFR_DIR") {
            bindgen_args.push(format!("-I{}/include", mpfr_dir));
            println!("cargo:rustc-link-search={}/lib", mpfr_dir);
        }
        
        // If on macOS, add homebrew paths for GMP and MPFR
        #[cfg(target_os = "macos")]
        {
            bindgen_args.push("-I/opt/homebrew/include".to_string());
            println!("cargo:rustc-link-search=/opt/homebrew/lib");
        }
        
        let bindgen_args_joined = bindgen_args.join(" ");
        println!("cargo:rustc-env=BINDGEN_EXTRA_CLANG_ARGS={}", bindgen_args_joined);
    }

    // Create bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("symengine_.*")  // Limit to symengine functions only
        .blocklist_type("max_align_t")  // Block problematic types
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}