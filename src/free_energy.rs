fn main() {
   let a: u32 = 1;
   let b = {
      let c = &a;
      unsafe { &*(c as *const u32) }
   };
   println!("{} {}", a, *b);
}
