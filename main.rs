use std::io::{self, BufRead, stdin};

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

fn main() {
    
    // Test 01
    /*let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let mut s = line.trim().to_string();
    append_excl(&mut s);
    println!("{}", s);*/

    // Test 02
    /*let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();

    println!("{}", longer(&a, &b));*/

    // Test 03
    /*let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let kind = lines.next().unwrap().unwrap();
    let dim: f64 = lines.next().unwrap().unwrap().parse().unwrap();

    let shape: Box<dyn Shape> = if kind == "circle" {
        Box::new(Circle { radius: dim })
    } else {
        Box::new(Square { side: dim })
    };

    println!("{:.2}", shape.area());*/

    // Test 04
    /*let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let numbers = line.split_whitespace();

    let sum_squares: i32 = numbers.map(|n| n.parse::<i32>().unwrap())
        .filter(|n| *n % 2 == 0)
        .map(|n| n * n)
        .sum();

    println!("{}", sum_squares);*/

    // Test 05
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();

    match parse_two(&a, &b) {
        Ok(v) => println!("sum: {}", v),
        Err(_) => println!("error: invalid input")
    }

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
