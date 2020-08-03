fn main() {
   let pair = (2, -2);

   println!("Tell me about {:?}", pair);
   match pair {
       (x, y) if x == y => println!("These are twins"),
       // The ^ `if condition` part is a guard
       (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
       (x, _) if x % 2 == 1 => println!("The first one is odd"),
       (x, y) if x + y != 0 => println!("No correlation..."),
       (_, y) if y % 2 == 1 => println!("The second one is odd"),
       (_, _) => unreachable!()
    }
}
