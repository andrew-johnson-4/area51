#[derive(Default)]
pub struct ABC {
   pub a: u64,
   pub b: u64,
   pub c: u64,
}

fn main() {
   let a = ABC { a:1, b:2, c:3 };
   let _ = ABC { a:2, ..a };
   let _ = ABC { a:3, ..std::default::Default::default() };
}
