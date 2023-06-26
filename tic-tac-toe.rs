fn main(){
    // let
    println!("Hey, Welcome to the Rust based Ston-Paper-pencile Game, Have a Fub ::))");

    println!("Here u should have to select Number according to Your chooses");

    // Now here Frontend team u should have to put alert for confirmation 
    // Then user able to move one page to another page

    println!("Here 1 for Stone, 2 for Paper, 3 for Pencil, 4 for Scissor");

    let first_user = get_input();
    let second_user = get_input();

    if first_user <=4 && second_user <= 4 
    {
    println!("First User chooses: {}",first_user);
    println!("First User chooses: {}",second_user);
    
    if first_user == second_user {
        println!("It's a tie!");
    }

    if (first_user == 1 && second_user == 2) || (first_user == 2 && second_user == 1) {
        println!("First player choice is: {}",first_user);
        println!("Second player choice is: {}",second_user);
        println!("Second player Win, And your choice is: {} ",second_user);
    }

    else if (first_user == 1 && second_user == 3) || (second_user == 1 && first_user == 2) {
        println!("First player choice is: {}",first_user);
        println!("Second player choice is: {}",second_user);
        println!("First player Win, And your choice is: {}",first_user);
    }
    else if (first_user == 1 && second_user == 4) || (first_user == 4 && second_user == 1) {
        println!("First player choice is: {}",first_user);
        println!("Second player choice is: {}",second_user);
        println!("First player Win, And your choice is: {}",first_user);
    }
    else if (first_user == 2 && second_user == 3) || (first_user == 3 && second_user == 2) {
        println!("First player choice is: {}",first_user);
        println!("Second player choice is: {}",second_user);
        println!("Second player Win, And your choice is: {} ",second_user);
    }
    else if (first_user == 2 && second_user == 4) || (first_user == 4 && second_user == 2) {
        println!("First player choice is: {}",first_user);
        println!("Second player choice is: {}",second_user);
        println!("Second player Win, And your choice is: {} ",second_user);
    }
    else if (first_user == 3 && second_user == 4) || (first_user == 4 && second_user == 3) {
        println!("First player choice is: {}",first_user);
        println!("Second player choice is: {}",second_user);
        println!("Second player Win, And your choice is: {} ",second_user);
    }
   } 

   else {
       println!("Please select value upto 4");
   }


}

fn get_input() ->i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number;
}
