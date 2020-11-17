fn main() {
    let p = Person {
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name();
    p.say_age();
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) {
        println!("I am {}.", self.name);
    }

    fn say_age(&self) {
        println!("I am {} year(s) old.", self.age);
    }
}
