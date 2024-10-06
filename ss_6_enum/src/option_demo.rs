fn plus_one(x:Option<i32>)->Option<i32>{
    match x{
        None => None,
        Some(i)=> Some(i+1),
    }
}

pub fn run(){
    let five = Some(5);
    let other_num = None;
    let six = plus_one(five);
    println!("six = {:?}",six);
    println!("other_num = {:?}",plus_one(other_num));
}