use std::{env, path::Path};

fn main() {
    let vendored = env::var("CARGO_FEATURE_VENDORED").is_ok();
    match vendored {
        true => {
            let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
            println!(
                "cargo:rustc-link-search=native={}",
                Path::new(&dir).join("qhyccd-sdk").join(arch).display()
            );
        }
        false => {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        }
    }
    println!("cargo:rustc-link-lib=static=qhyccd");
    println!("cargo:rustc-link-lib=dylib=usb-1.0");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
