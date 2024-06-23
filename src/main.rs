fn main() {
    let str = String::from("not me");
    let s2 = str;
    // print_something(s2);
    println!("{}", str);
    let student= Student{
        name: String::from(""),
        age: 12,
        height: 5.2
    };
    println!("{}", student+"student me: ")
}

fn print_something(p0: String) {
    println!("{p0}")
}


enum ROLE {
    ADMIN,
    STUDENT,
    TEACHER,
}

struct Student {
    name: String,
    age: u32,
    height: f32
}