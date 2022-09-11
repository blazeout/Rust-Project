

use front_of_house::Student;


mod front_of_house;

pub fn wanna_student() {
    let mut s = Student::get_student(String::from("王嘉豪"), 18);
    println!("{},  {}", s.age, s.name);
    s.height = 180;
}

pub fn eat() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}