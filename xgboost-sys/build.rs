fn main() {
    let s = std::process::Command::new("make").arg("-j").arg("config=make/minimum.mk")
        .current_dir("xgboost")
        .status()
        .expect("Failed to execute XGBoost make script.");
    assert!(s.success());

    bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Ixgboost/include")
        .clang_arg("-Ixgboost/rabit/include")
        .generate()
        .expect("Unable to generate bindings.")
        .write_to_file(format!("{}/bindings.rs", std::env::var("OUT_DIR").unwrap()))
        .expect("Couldn't write bindings.");

    println!("cargo:rustc-link-search=xgboost/lib");
    println!("cargo:rustc-link-search=xgboost/rabit/lib");
    println!("cargo:rustc-link-search=xgboost/dmlc-core");
    println!("cargo:rustc-link-lib=static=rabit_empty");
    println!("cargo:rustc-link-lib={}", if std::env::var("TARGET").unwrap().contains("apple") { "c++" } else { "stdc++" });
    println!("cargo:rustc-link-lib=static=dmlc");
    println!("cargo:rustc-link-lib=static=xgboost");
}
