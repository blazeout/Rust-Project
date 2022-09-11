pub mod hosting {
    pub fn add_to_waitlist() {}
}

pub struct Student {
    pub name: String,
    pub age: i32,
    pub  height: i32,
}

impl Student {
    // 构造方法, 构造器 不带self
    pub fn get_student(name: String, age: i32) -> Student {
        Student { 
            name,
            age,
            height: 175,
        }
    }
}