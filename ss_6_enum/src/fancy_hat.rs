pub fn run(){
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(),
    }

}

fn add_fancy_hat(){
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat(){
    println!("Removing a fancy hat!");
}

fn move_player(){
    println!("Moving player!");
}