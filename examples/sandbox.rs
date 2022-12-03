use std::fmt::format;

fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}

fn dump(s: &String) {
    println!("{}", s);
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn copy(&self) -> Self {
        Self::new(&self.first_name, &self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }
}

#[derive(Debug)]
struct A <'a> {
    s: &'a str,
}

fn how(i: u32) -> &'static str {
    match i {
        0 => "none",
        1 => "one",
        _ => "many"
    }
}

fn main() {
    let s1 = "hello dolly".to_string();
    dump(&s1);
    println!("{}", s1);

    for i in 0..10 {
        println!("{}", i);
    }
    let names = ["alice", "bob", "charles"];
    for s in ["a", "b", "c"].iter().zip(names.iter()) {
        println!("{} {}", s.0, s.1);
    }

    let person = Person::new("John", "Smith");
    println!("person {}", person.full_name());
    println!("{:?}", person);
    println!("{:?}", person.to_tuple());
    print!("hello world!!!!!!!!!!!!");

    let s = "I'm a little string".to_string();
    let a = A {s: &s};
    println!("{:?}", a);
}