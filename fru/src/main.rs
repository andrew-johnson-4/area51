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

   let a = area51_fru::ABC { a:1, b:2, c:3 };
   let _ = area51_fru::ABC { a:2, ..a };
   let _ = area51_fru::ABC { a:3, ..std::default::Default::default() };

   let a = area51_fru::fru::ABC { a:1, b:2, c:3 };
   let _ = area51_fru::fru::ABC { a:2, ..a };
   let _ = area51_fru::fru::ABC { a:3, ..std::default::Default::default() };
}

