#[cfg(target_os = "windows")]
mod console_style {
    use std::os::raw::{c_long, c_ushort, c_ulong, c_uint, c_void, c_ushort as c_wchar};

    #[repr(C)]
    struct COORD {
        x: c_ushort,
        y: c_ushort,
    }

    #[repr(C)]
    struct CONSOLE_FONT_INFOEX {
        cb_size: c_ulong,
        n_font: c_ulong,
        dw_font_size: COORD,
        font_family: c_uint,
        font_weight: c_uint,
        face_name: [c_wchar; 32],
    }

    extern "system" {
        fn GetStdHandle(nStdHandle: c_ulong) -> *mut c_void;
        fn SetCurrentConsoleFontEx(hConsoleOutput: *mut c_void, bMaximumWindow: c_long, lpConsoleCurrentFontEx: *mut CONSOLE_FONT_INFOEX) -> c_long;
    }

    pub fn set() {
        unsafe {
            std::process::Command::new("cmd").args(["/C", "color 0A"]).status().ok();
            
            let handle = GetStdHandle(0xFFFFFFF5); // STD_OUTPUT_HANDLE = -11
            let mut font = CONSOLE_FONT_INFOEX {
                cb_size: std::mem::size_of::<CONSOLE_FONT_INFOEX>() as c_ulong,
                n_font: 0,
                dw_font_size: COORD { x: 0, y: 20 },
                font_family: 54, 
                font_weight: 700, 
                face_name: [0; 32],
            };
            
            let face = "Consolas";
            for (i, c) in face.encode_utf16().enumerate() {
                font.face_name[i] = c;
            }
            
            SetCurrentConsoleFontEx(handle, 0, &mut font);
        }
    }
}

fn main() {
    #[cfg(target_os = "windows")]
    console_style::set();

    println!("Testing font and color!");
}
