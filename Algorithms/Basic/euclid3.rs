fn gcd(mut x: u64, mut y: u64) -> u64 {

   assert!(x != 0 && y != 0);

   while y != 0 {

         if y < x {

             let t = y;
                 y = x;
                 x = t; 
         }
         y = x % y;
   }

   x
}

fn main() {

   let x = 10;
   let y = 4; 

   println!("gcd ({}, {}) = {}", x, y, gcd(x, y));
   
}
