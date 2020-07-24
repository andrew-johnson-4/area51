use std::os::raw::{c_char,c_int};
use std::ffi::CString;

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}

fn main() {
   let f = CString::new("%d %d").expect("CString::new failed");
   unsafe {
      printf(f.as_ptr(), 1, 2);
   }
}
