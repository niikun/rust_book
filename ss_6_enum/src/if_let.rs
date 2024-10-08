

pub fn run(){
    let dice_roll = 9;

    add_fancy_hat(dice_roll);
}

fn add_fancy_hat(num:i32){
    if num == 3 {
        println!("Adding a fancy hat!");
    } else {
        println!("Moving player!");
    }
}