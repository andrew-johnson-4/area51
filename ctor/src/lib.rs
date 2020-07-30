use ctor::ctor;
use std::sync::atomic::{AtomicBool,Ordering};

pub static INITED: AtomicBool = AtomicBool::new(false);

#[ctor]
fn foo() {
   INITED.store(true, Ordering::SeqCst);
}
