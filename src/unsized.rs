struct Cell<T: ?Sized> {
   pub block: T
}

fn main() {
   let s = &[0, 1, 2, 3];
   let _ = Cell {
      block: s
   };
}
