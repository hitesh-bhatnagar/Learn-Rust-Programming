use std::env;	// gives all cli args

// to run the program 
//		cargo run -- 6.9 + 6.9

fn help(){
	println!("Usage: Calc_cli <number1> <operator> <number2> ");
	println!("Examples : Calc_cli 5 + 3");
	println!("supported operators: + - * / % ^");
}
fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() != 4 {
		help();
		return;
	}

	let left_txt = &args[1];
	let operator = &args[2];
	let right_txt = &args[3];

	// parse::<f64>() converts txt to floating pt number
	let left = match left_txt.parse::<f64>() {
		Ok(value) => value,
		Err(_) => {
			eprintln!("Error: '{}' is not a valid number.", left_txt);
			return;
		}
	};

	let right = match right_txt.parse::<f64>(){
		Ok(value) => value,
		Err(_) =>{
			eprintln!("Error: '{}' is not a valid number.", right_txt);
			return;
		}
	};

    let result = match operator.as_str() {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0.0 {
                Err("division by zero")
            } else {
                Ok(left / right)
            }
        }
        "%" => {
            if right == 0.0 {
                Err("modulo by zero")
            } else {
                Ok(left % right)
            }
        }
        "^" => Ok(left.powf(right)),
        _ => Err("unknown operator"),
    };

	match result {
		Ok(value) => println!("Result: {}", value),
		Err("division by zero") => eprintln!("Error: division by zero."),
		Err("modulo by zero") => eprintln!("Error: modulo by zero"),
		Err("unknown operator") => {
			eprintln!("Error: unknown operator '{}'", operator);
			help();
		}

		Err(other) => eprintln!("Error: {}", other),
	}
}
