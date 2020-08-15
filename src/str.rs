
fn main() {
   let mut s = String::new();
   let rs = &s;
   s.push('a');
   println!("s = '{}'", s);
   println!("rs = '{}'", rs);
}
