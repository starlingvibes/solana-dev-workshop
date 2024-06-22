struct Student {
    name: String,
    age: u8,
}

fn main() {
    println!("Hello, world");
}

fn create_student(name: String, age: u8) -> Student {
    Student { name, age }
}