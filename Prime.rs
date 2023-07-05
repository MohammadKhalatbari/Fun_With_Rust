// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of 600851475143?
fn main() {
    let mut input = String::new();
    let mut prime_list: Vec<u128> = Vec::new();
    println!("Please enter a number:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if let Ok(_input_number) = input.trim().parse::<u128>() {
        let mut input_number: u128 = input.trim().parse().unwrap();
        let mut i = 2;
        while i <= input_number {
            if input_number % i == 0 {
                prime_list.push(i);
                input_number /= i;
            } else {
                i += 1;
            }
        }
    } else {
        println!("Invalid input");
    }

     println!("{:?}", prime_list);
}