use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Please enter a number:");
    // Read a line of text from the user
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Parse the input string as a u32
            let input_number: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input");
                    return;
                }
            };
            
            fibbonacci(input_number);
        }
        Err(error) => {
            // If there was an error reading the input, handle it
            println!("Error: {}", error);
        }
    }
}

fn fibbonacci(input_number: u32) {
    let mut my_list: Vec<u32> = Vec::new();
    let mut i: u32 = 0;
    let mut sum: u32 = 0;
    while true{
        if i == 0 {
            my_list.push(0);
        }
        else if i == 1 || i == 2 {
            my_list.push(1);
        }
        else {
            let mut temp: u32 = my_list[(i-1) as usize] + my_list[(i-2) as usize];
            my_list.push(temp);
            if temp > input_number {
                false;
                break;
            }
        }
        i = i + 1;
    }
    for i in 0..my_list.len() {
        if my_list[i] % 2 == 0 {
            sum = sum + my_list[i];
        }
       
    }
    println!("{:?}" , sum)
}

