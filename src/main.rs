use std::io; // User input 




fn main(){
    
    let mut user_input =  String::new(); // variable to get user input
    let mut option:i32 ;

    println!("\nWelcome back Omar to myTasks Notes!");
    loop{
        
        println!("Choose one of the following options: ");
        println!("\t1) View current tasks.");
        println!("\t2) Create new task.");
        println!("\t3) Change priority.");
        
        io::stdin().read_line(&mut user_input).expect("**ERROR: failed to readline"); // reads user input as a string

        option = user_input.trim().parse().expect("*ERROR: Please, enter a valid number.");

        if (1..=3).contains(&option) {
            
            match option{
                1 => println!("HELLO1"),
                2 => println!("HELLO2"),
                3 => println!("HELLO3"),
                _ => println!("OTHER"),
            }

        }else {
            println!("**ERROR: Please, enter a valid option");
        }
    }

}
