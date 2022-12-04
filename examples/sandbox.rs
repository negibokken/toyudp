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

#[derive(Debug, PartialEq)]
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

#[derive(PartialEq,PartialOrd)]
enum Speed {
    Slow = 10,
    Medium = 20,
    Fast = 50
}

enum Difficulty {
    Easy = 1,
    Medium,
    Hard,
}

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool)
}

fn dump3(v: &Value) {
    use Value::*;
    match v {
        Number(n) => println!("number is {}", n),
        Str(ref s) => println!("string i '{}'", s),
        Bool(b) => println!("boolean is {}", b)

    }
}

impl Value {
    fn to_str(self) -> Option<String>{
        match self {
            Value::Str(s) => Some(s),
            _ => None
        }
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn match_tuple(t: (i32, String)) {
    let text = match t {
        (0,s) => format!("zero {}", s),
        (1, ref s) if s == "hello" => format!("hello one!"),
        tt => format!("no match {:?}",tt),
    };
    println!("{}", text);
}

type NodeBox = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    payload: String,
    left: NodeBox,
    right: NodeBox,
}

impl Node {
    fn new (s: &str) -> Node {
        Node {payload: s.to_string(), left: None, right:None}
    }

    fn boxer (node:Node) -> NodeBox {
        Some(Box::new(node))
    }

    fn set_left (&mut self, node: Node)  {
        self.left = Self::boxer(node);
    }

    fn set_right (&mut self, node: Node)  {
        self.right = Self::boxer(node);
    }

    fn insert(&mut self, data: &str) {
        if data < &self.payload {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }

    fn visit(&self) {
        if let Some(ref left) = self.left {
            left.visit()
        }
        println!("{}", self.payload);
        if let Some(ref right) = self.right {
            right.visit();
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

    assert_eq!(d, Direction::Left);

    let s = Speed::Slow;
    let speed = s as u32;
    println!("speed {}", speed);

    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);

    dump3(&n);
    dump3(&s);
    dump3(&b);
    println!("s? {:?}", s.to_str());

    let p = Point { x: 1.0, y: 2.0 };
    let Point {x,y} = p;
    println!("x: {}, y: {}, p: {:?}", x,y,p);

    let f =  |x| x*x;
    let res = f(10);
    println!("res: {}", res);

    let name = "dolly".to_string();
    let age = 42;

    let cname = name.to_string();

    let c = move || {
        println!("name {} age {}", cname, age);
    };
    c();
    println!("name {}",name);

    let mut root = Node::new("root");
    root.set_left(Node::new("left"));
    root.set_right(Node::new("right"));

    println!("{:?}", root);

    let mut root2 = Node::new("root");
    root2.insert("one");
    root2.insert("two");
    root2.insert("four");

    println!("root {:#?}", root2);
    println!("root hello");
    root.visit();
}
