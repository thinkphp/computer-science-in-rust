
fn gcd(mut x: u64, mut y: u64) -> u64 {

   assert!(x != 0 && y != 0);

   while y != 0 {

   	     let t = x % y;
   	     x = y;
   	     y = t;
   }   
   x
}

fn get_input() -> String {

   let mut buffer = String::new();

   std::io::stdin().read_line(&mut buffer).expect("Failed");

   buffer
}

fn main() {
	
  let x = get_input().trim().parse::<u64>().unwrap();
  let y = get_input().trim().parse::<u64>().unwrap();
  
  println!("Euclid ({:?}, {:?}) -> {:?}", x, y, gcd(x,y));
} 
