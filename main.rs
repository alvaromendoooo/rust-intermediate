use std::io::{self, BufRead, Read, stdin};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

trait Shape { fn area(&self) -> f64; }
struct Circle { radius: f64 }
struct Square { side: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 {
        let pi = 3.14;
        pi * self.radius.powf(2.0)
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

mod geometry {
    pub fn circle_area(radius: f64) -> f64 {
        3.14 * radius * radius
    }

    pub fn square_area(side: f64) -> f64 {
        side * side
    }
}

fn main() {
    
    // Test 01
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let mut s = line.trim().to_string();
    append_excl(&mut s);
    println!("{}", s);

    // Test 02
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();

    println!("{}", longer(&a, &b));

    // Test 03
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let kind = lines.next().unwrap().unwrap();
    let dim: f64 = lines.next().unwrap().unwrap().parse().unwrap();

    let shape: Box<dyn Shape> = if kind == "circle" {
        Box::new(Circle { radius: dim })
    } else {
        Box::new(Square { side: dim })
    };

    println!("{:.2}", shape.area());

    // Test 04
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let numbers = line.split_whitespace();

    let sum_squares: i32 = numbers.map(|n| n.parse::<i32>().unwrap())
        .filter(|n| *n % 2 == 0)
        .map(|n| n * n)
        .sum();

    println!("{}", sum_squares);

    // Test 05
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();

    match parse_two(&a, &b) {
        Ok(v) => println!("sum: {}", v),
        Err(_) => println!("error: invalid input")
    }

    // Test 06
    let counter = Rc::new(RefCell::new(0));
    let a = Rc::clone(&counter);
    let b = Rc::clone(&counter);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 1;

    println!("{}", counter.borrow());

    // Test 07
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.split_whitespace().next().unwrap().parse().unwrap();
    let mut c = make_counter();
    for _ in 0..n {
        println!("{}", c());
    }

    // Test 08
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let kind: String = lines.next().unwrap().unwrap();
    let dim: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let area = if kind == "circle" {
        geometry::circle_area(dim)
    } else {
        geometry::square_area(dim)
    };

    println!("{:.2}", area);

    // Test 09
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handlers.push(thread::spawn(move || {
            let mut c = counter.lock().unwrap();
            *c += 250;
        }));
    }

    for h in handlers { h.join().unwrap(); }
    println!("{}", *counter.lock().unwrap());

}

fn append_excl(s: &mut String) {
    s.insert(s.len(), '!');
}

fn longer<'t>(a: &'t str, b: &'t str) -> &'t str {
    if a.len() >= b.len() {
        return a
    } else {
        return b
    }
}

fn parse_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let num_a = a.parse::<i32>()?;
    let num_b = b.parse::<i32>()?;

    Ok(num_a + num_b)
}

fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;

    move || {
        count += 1;
        count
    }
}
