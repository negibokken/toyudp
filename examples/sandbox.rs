use std::fmt;

fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}

fn dump(s: &String) {
    println!("{}", s);
}

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

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show (&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show (&self) -> String {
        format!("eight-byte float {}", self)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.full_name())
    }
}


struct FRange {
    val: f64,
    end: f64,
    incr: f64
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {val: x1, end: x2, incr: skip}
}

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}

fn dump2<T>(value: & T) where T: std::fmt::Debug {
    println!("value is {:?}", value);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        }
    }

    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
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
    // println!("{:?}", person.to_tuple());
    print!("hello world!!!!!!!!!!!!");

    let s = "I'm a little string".to_string();
    let a = A {s: &s};
    println!("{:?}", a);

    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);
    println!("{:?}", person);

    for x in range(0.0, 1.0, 0.1) {
        println!("{}", x);
    }
    let v: Vec<f64> = range(0.0, 1.0,0.1).map(|x| x.sin()).collect();
    println!("{:?}",v);

    let n = 42;
    dump2(&n);
    let start = Direction::Left;
    println!("{:?}",start);
    let mut d = start;
    for _ in 0..8 {
        println!("d {:?}", d);
        d = d.next();
    }
}
