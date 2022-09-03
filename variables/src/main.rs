const MAX_POINTS: i32 = 100_000;

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("{}", x);
    println!("const is {}", MAX_POINTS);

    let x = x + 1;
    let mut x = x*2;
    x += 1;
    println!("x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("length is {}", spaces);
}

