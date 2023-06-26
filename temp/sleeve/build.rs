extern crate winres;

use std::path::Path;


fn main() {
    if cfg!(target_os = "windows") {
        let file_path = Path::new("ck.ico");
        if file_path.exists() {
            let mut res = winres::WindowsResource::new();
            res.set_icon("ck.ico");
            res.compile().unwrap();
        }
    }
}