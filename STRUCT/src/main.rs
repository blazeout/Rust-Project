fn main() {
    
    let mut user = User{
        email: String::from("1072830@qq.com"),
        username: String::from("Nikky"),
        active: true,
        sign_in_count: 60,
    };
    println!("{}", user.email);
    user.active = false;
    let  user2 = build_user(user.email, user.username);
    let _user3 = User {
        email: String::from("123132"),
        username: String::from("nihao"),
        ..user2
    };
    let _black = Color(0,0);
    // 元组用 点 访问 数组用下标方括号访问

    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", rect.area());

    println!("{:#?}", rect);

    let rect2 = Rectangle {
        width: 20,
        length : 50,
    };
    let canhold = rect.can_hold(&rect2);
    println!("{}", canhold);

    let square1 = Rectangle::square(10);
    println!("{:?}", square1);

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width >= other.width && self.length >= other.length {
            return true;
        }
        false
    }

    // 关联函数 构造函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}



fn build_user(email: String, username: String) -> User {
    User { username, email, sign_in_count: (60), active: (false) }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32);

