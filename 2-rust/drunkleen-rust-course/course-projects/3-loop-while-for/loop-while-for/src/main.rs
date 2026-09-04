use std::io::stdin;

fn main() {

    //Loop
    //lloop("hello world");

    //while loop
    //wloop("hello world");

    //for loop
    //forloop("hello world");

    let mut number: Vec<i32> = Vec::new();
    loop {
        println!("hit 0 to exit or give a number");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("number is not invaled {}", e);
                continue;
            }
        };
        
        if input == 0 {
            break;
        }
        number.push(input);
    }
    
    let mut even_count: i32 = 0;
    let mut index: i32 = 0;
    
    while index < number.len() as i32 {
        if number[index as usize] % 2 == 0 {
            even_count += 1;
        }
        index += 1;
    }
    
    let mut summ: i32 = 0;
    for i in number {
        summ += i;
    }
    println!("Sum: {}", summ);
    println!("Even count: {}", even_count);
    println!("exit");
    
}
 
fn lloop(name: &str) {
    let mut counter: i32 = 0;
    while counter < 100 {
        println!("{}-{}", counter, name);
        counter += 1;
    }
}

fn wloop(name: &str) {
    let mut counter: i32 = 0;
    while counter < 100 {
        println!("{}-{}", counter, name);
        counter += 1;
    }
}

fn forloop(name: &str) {
    for counter in 0..100 {
        println!("{}-{}", counter, name);
    }
}
