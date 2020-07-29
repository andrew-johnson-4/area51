use ctor::ctor;
use std::sync::atomic::{AtomicBool,Ordering};

static INITED: AtomicBool = AtomicBool::new(false);

#[ctor]
fn foo() {
   INITED.store(true, Ordering::SeqCst);
}

fn main() {
   println!("main {}", INITED.load(Ordering::SeqCst));
}
