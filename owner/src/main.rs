
fn main() {
    let s = "123";
    // move Rust为了避免二次释放 直接将str 给move掉了 失效了 只留下str2是生效的 这样就保证了安全性
    let  str = String::from("hello");
    let str2 = str;
    let str3 = str2;
    // println!("{}, {}, {}", str, str2, s2);
    println!("{}, {}", s, str3);

    // 将值传递给函数, 和 赋值给变量是差不多的 要么发送移动或者是拷贝

    let mut s1 = String::from("s");
    // print_string(s1);
    let number = 1;
    print_number(number);

    // 我们可以传递 引用 即取地址来保证不获取其所有权, 可变引用
    let length = calcucate_length(&mut s1);
    println!("the string {} length is {}", s1, length);

    let s2 = &mut s1;
    // 可以存在多个不可变的引用. 可变的引用只能存在一个
    let s3 = String::from("Hello world");
    let first_words = first_word(&s3);
    println!("index is {}", first_words);
    let hello = &s3[0..5];
    let world = &s3[6..11];
    println!("{}, {}", hello, world);

    let abc = [1,2,3,4,5];
    let sliceabc = &abc[..2];
    for (i, val) in sliceabc.iter().enumerate() {
        println!("{} {}", i, val);
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for i in 0 .. s.len() {
        if bytes[i] == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn calcucate_length(s: &mut String) -> usize {
    s.push_str("abc");
    s.len()
}

// fn print_string(str: String) {
    
//     println!("{}", str);
// }

fn print_number(number: i32) {
    println!("{}", number);
}
