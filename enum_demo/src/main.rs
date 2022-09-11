fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    route(v4);
    route(v6);
    route(IpAddrKind::V4);
    let m = Message::Move { x: (1), y: (2) };

    let x = Some(5);
    let y:Option<u32> = None;
    let z = 10;
    println!("{}, {}", values_in_coins(Coin::Penny), values_in_coins(Coin::Dime));

    let v = 0u8;

    match v {
        0 => println!("zero"),
        1 => println!("one"),
        _ => (),
    }

    if let 0 = v {
        println!("{}", v);
    }
}

fn values_in_coins(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("{}", String::from("println String")); 
            5 + 5
        },
        Coin::Quarter => 25,
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {

}
