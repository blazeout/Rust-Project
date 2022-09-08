
fn main() {
    // println!("Hello, world!");
    // another_function(5, 8);

    // let _y = {
    //     let x = 1;
    //     x + 3
    // };

    // let x = plus_five(6);
    // println!("{}", x);

    // let number = 4;
    // if number < 5 {
    //     println!("the number is {}", number)
    // }else {
    //     println!("haall")
    // }

    
    // let x = 4;
    // match x {
    //     1 => println!("1"),
    //     4 => println!("4"),
    //     _ => println!("None")
    // };

    loop_range();
    while_range();
    for_range();
}

// 函数的参数必须标明类型
fn another_function(x: i32, y: i32) {
    println!("this is another function");
    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
}


fn plus_five(x: i32) -> i32 {
    5 + x
}

fn for_range() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("element is {}", element);
    }
    // range 
    for i in (0 .. 4).rev() {
        println!("{}", i);
    }
}

fn while_range() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("{}", a[index]);
        index = index+1;
    }
}

fn loop_range() {
    let mut x = 1;
    loop {
        println!("hello, {}", x);
        x += 1;
        if x == 10 {
            break;
        } 
    }
}
