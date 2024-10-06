#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr{
    kind :IpAddrKind,
    address:String,
}

pub fn run(){
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("192.0.0.1")
    };
    println!("{:?}",home);
}