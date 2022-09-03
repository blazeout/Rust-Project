use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数游戏开始了!!!");
    println!("请猜测一个数");
 
    let secret_number = rand::thread_rng().gen_range(1, 101); // i32 u32 i64
    // println!("神秘數字是: {}", secret_number);
    loop {
        println!("请猜数!");
        let mut guess_string = String::new();
        io::stdin().read_line(&mut guess_string).expect("无法读取行");
        // io::Result 是read_line的返回 有两种 Ok 和 Err
        println!("你读取的数是: {}", guess_string);
        // 返回是一个枚举 可以使用match来解决
        let guess:u32 = match guess_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too samll"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }        
    }
}
