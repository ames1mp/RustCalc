/*******************************************************************************
@author Mike Ames
@version Fall 2017
*******************************************************************************/
use std::io;
use std::f32;


/*******************************************************************************
Main driver function. Prints the welcome message, and calls the function to 
collect and process user input.
@version Fall 2017
*******************************************************************************/
fn main() {
    print_welcome_msg();
    
    // Running total
    let mut total: f32 = 0.0;
    
    loop {
        println!("The total is {}, please enter an arithmetic operation, or 'Q to quit.", total);
        
        //user input
        let mut user_in = String::new();
        user_in = user_in.parse().expect("Please enter an operation or 'Q' to quit.");
        
        //operand parsed from user input
        //let operand: f64 = 0.0;

        //operator parsed from user input
        let operator = String::new();

        io::stdin().read_line(&mut user_in).expect("Failed to read line");
        user_in.trim();

        if "Q".to_string() == user_in || "q".to_string() == user_in {
            break;
        }

        let user_in = user_in;
        let tokens: Vec<&str> = user_in.split_whitespace().collect();
        
        if tokens.len() > 2 {
            println!("Please limit inputs to one operator and one operand, seperated by whitepsace, or 'Q' to quit.");
            continue;
        }
            let operand: f32 = tokens[1].parse().unwrap();
            
            match tokens[0] {
            "+" => total += operand,
            "-" => total -= operand,
            "*" => total *= operand,
            "/" => total /= operand,
            "^" => total = total.powf(operand),
            _ => println!("No Operand Found")
            }
        

        


    }
    
    print_goodbye_msg(total);
}


/*******************************************************************************
@author Mike Ames
@version Fall 2017
*******************************************************************************/
fn print_welcome_msg() {
    println!("Welcome to the calculator program.");
    println!("This program allows you to enter arithmetic operations.");
    println!("A running total is kept and is modified by the operations you enter");
    println!("For example, if the total is 12 and you enter '+ 3', the new total will be 15.");
    println!("If the total is 2 and you enter '^ 3', the new total will be 8.");
    println!("Supported operations are: '+' '-' '*' '/' and '^'");
    println!("Please limit inputs to one operator followed by one operand.");
    println!("You may enter 'Q' at any time to quit.");
}

fn print_goodbye_msg(total: f32) {
    println!("Your final total was: {}, Goodbye!", total);
}