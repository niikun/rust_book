#[derive(Debug)]
struct User{
    email:String,
    username:String,
    active:bool,
    sign_in_count:i32,
}

#[derive(Debug)]
struct Color(i32,i32,i32);

#[derive(Debug)]
struct Point(i32,i32,i32);

pub fn run(){
    let mut user1 = User{
        email:String::from("niikun@example.com"),
        username:String::from("niikun"),
        active:true,
        sign_in_count:1
    };

    user1.email = String::from("niikun@example.jp");
    println!("{:?}",user1);

    let user2 = build_user(
        String::from("sankun@example.com"),
        String::from("sankun")
    );
    println!("{:?}",user2);

    let color = Color(255,0,0);
    let point = Point(1,1,1);

    println!("color: {:?}, point :{:?}",color,point);

}

fn build_user(email:String,username:String)->User{
    User{
        email,
        username,
        active:true,
        sign_in_count:1
    }
}
