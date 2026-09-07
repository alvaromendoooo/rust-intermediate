use std::io::{self, BufRead, stdin};

// fn append_excl(s: &mut String) { ... }

fn main() {
    
    // Test 01
    /*let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let mut s = line.trim().to_string();
    append_excl(&mut s);
    println!("{}", s);*/

    // Test 02
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();

    println!("{}", longer(&a, &b));
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
