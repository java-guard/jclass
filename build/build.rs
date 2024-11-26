use std::env;
use std::path::PathBuf;

fn main() {
    let header_path = "include/classfile_constants.h";
    let bindings = bindgen::Builder::default()
        .header(header_path)
        .generate()
        .expect("无法生成头文件对应Rust代码");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("classfile_constants.rs"))
        .expect(&format!("无法写出生成的Rust代码：{}", &out_path.to_str().unwrap()));
}