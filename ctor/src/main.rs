use ctor::ctor;
use std::sync::atomic::{AtomicBool,Ordering};

static INITED: AtomicBool = AtomicBool::new(false);

#[ctor]
fn foo() {
   INITED.store(true, Ordering::SeqCst);
}

fn main() {
   #[ctor]
   fn f() {}
   println!("lib {}", area51_ctor::INITED.load(Ordering::SeqCst));
   println!("main {}", INITED.load(Ordering::SeqCst));
}
