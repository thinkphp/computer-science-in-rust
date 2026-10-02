use std::io::{self, Write};

fn main() {

	enum Solution {
		Unique(f64),
		Infinite,
		None
	}

	//solve the equation a * x + b = 0
	fn solve_linear(a: f64, b: f64) -> Solution {

             if a != 0.0 {

             	let x = - b/a;

             	//evitam afisarea lui -0.0
             	Solution::Unique(if x == -0.0 {0.0} else {x})

             }  else if b == 0.0 {

             	Solution::Infinite

             } else {

                Solution::None
             }
	}

	fn read_f64(prompt: &str) -> f64 {

       loop {

       	    print!("{}", prompt); 
       	    io::stdout().flush().expect("Eroare la flush stdout");

       	    let mut input = String::new();

       	    io::stdin().read_line(&mut input).expect("Eroare la citire");

       	    match input.trim().parse::<f64>() {

       	    	Ok(num) => return num,
       	    	
       	    	Err(_) => println!("Invalid value! Enter a valid number."),
       	    }
       }    

	}
	//functie helper pentru citirea si validarea unui numar f64 de la tastatura

	println!("Linear Equation Resolver");
	println!("Pentru ecuatia de forma 'a * x + b = 0': \n");

	let a = read_f64("a = ");
	let b = read_f64("b = ");

	match solve_linear(a, b) {

		Solution::Unique(x) => println!("Solutie unica: x = {:.4}", x),
		Solution::Infinite => println!("Ecuatia admite o infinitate de solutii"),
		Solution::None => println!("Ecuatia nu are nicio solutie")
	}

}
