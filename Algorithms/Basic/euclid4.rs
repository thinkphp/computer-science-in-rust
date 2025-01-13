use std::io;
use std::str::FromStr;
use std::cmp::Ordering;

fn get_number(prompt_input: &str) -> u32 {

       println!("{}", prompt_input);
       
       let mut input = String::new();
       
       io::stdin().read_line(&mut input).expect("no input!");     
       
       u32::from_str(input.trim()).unwrap()      
}

fn main() {
     
       let a = get_number("Enter first number a = ");
       
       let b = get_number("Enter first number b = ");

       println!("The Greatest Common Divisor of {} and {} is {}", a, b, euclid( a, b) );       
}

fn euclid(a: u32, b: u32) -> u32 {
 
   assert!(a > 0 && b > 0);
   
   match a.cmp(&b) {
   
      Ordering::Equal => b,
      
      Ordering::Less => euclid(a,b-a),
      
      Ordering::Greater => euclid(a-b,b)
   }   
}
