use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
   pub static ref LIST: Mutex<Vec<u64>> = Mutex::new(Vec::new());
   pub static ref L1: u64 = {
      LIST.lock().unwrap().push(1);
      1
   };
}

fn main() {
   println!("{:?}", LIST.lock().unwrap());
}
