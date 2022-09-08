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
    // shadowing 隐藏 代表可以使用同名变量去重复声明一个变量 覆盖掉
    let spaces = "    ";
    let spaces = spaces.len();
    println!("length is {}", spaces);
    // 中文字符占3个字节长度 因为使用的是UTF-8编码
    let words = "你好呀";
    println!("length of words is {}", words.len());

    let guess: u32 = "42".parse().expect("not a number");
    println!("{}", guess);
    // 数据类型 整数 浮点数 布尔 字符 char
    let a = 5;
    let b = 2.0;
    let b = true;
    let f = false;
    let c = 'c';
    let x = '%';
    // 元组
    let tup = (1, 1.1, true);
    let (x, y, z) = tup;
    println!("{}{}{}", x, y, z);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let a: [i32; 5] = [1,2,3,4,5];
    // 另外一种声明方式
    let b = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{}, {}", a[0], b[0]);
}



