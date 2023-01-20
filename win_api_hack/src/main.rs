use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
   
    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("World"), MB_OK);
    }

    Ok(())
}
