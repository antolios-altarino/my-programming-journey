use rand::random_range;

fn main() {
    rock_paper_scissors_if(2);
    rock_paper_scissors_match(1);
}

/* Rock Paper Scissors game with if condition*/
fn rock_paper_scissors_if(my_choice: u8) -> u8 {
    println!("Welcome to rock paper scissors");
    println!("1. rock");
    println!("2. paper");
    println!("3. scissors");
    
    let enemy_choice: u8 = random_range(1..4);
    
    if my_choice == 1 {
        println!("your choice is: rock");
    } else if my_choice == 2 {
        println!("your choice is: paper");
    } else {
        println!("your choice is: scissors");
    }

    if enemy_choice == 1 {
        println!("enemy choice is: rock");
    } else if enemy_choice == 2 {
        println!("enemy choice is: paper");
    } else {
        println!("enemy choice is: scissors");
    }

    if my_choice == enemy_choice {
        println!("draw");
        1
    } else if my_choice == 1 && enemy_choice == 2 {
        println!("you lose");
        2
    } else if my_choice == 1 && enemy_choice == 3 {
        println!("you win");
        1
    } else if my_choice == 2 && enemy_choice == 1 {
        println!("you win");
        1
    } else if my_choice == 2 && enemy_choice == 3 {
        println!("you lose");
        2
    } else if my_choice == 3 && enemy_choice == 1 {
        println!("you lose");
        2
    } else if my_choice == 3 && enemy_choice == 2 {
        println!("you win");
        1
    } else {
        println!("error");
        4
    }
    
}

/* Rock Paper Scissors game with match condition*/
fn rock_paper_scissors_match(my_choice: u8) -> u8 {
    println!("Welcome to rock paper scissors");
    println!("1. rock");
    println!("2. paper");
    println!("3. scissors");
    
    let enemy_choice: u8 = random_range(1..4);
    
    match my_choice {
        1 => println!("your choice is: rock"),
        2 => println!("your choice is: paper"),
        3 => println!("your choice is: scissors"),
        _ => println!("invalid choice"),
    }

    match enemy_choice {
        1 => println!("enemy choice is: rock"),
        2 => println!("enemy choice is: paper"),
        3 => println!("enemy choice is: scissors"),
        _ => println!("invalid choice"),
    }

    match (my_choice, enemy_choice) {
        (x, y) if x == y => {
            println!("draw");
            1
        },
        (1, 2) | (2, 3) | (3, 1) => {
            println!("you lose");
            2
        },
        (1, 3) | (2, 1) | (3, 2) => {
            println!("you win");
            1
        },
        _ => {
            println!("error");
            4
        }
    }
}


