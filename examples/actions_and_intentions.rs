use std::io::stdin;
use diesel::Connection;
use point::Point3D;
use point::Point2D; // Cmd+Alt+O to optimize imports.

fn main() {
    // Replace .unwrap() with match
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap/*caret*/(); // Alt+Enter -> Replace .unwrap() with match

    // Split into 2 if's
    if s.len() == 2 /*caret*/&& s.starts_with("x") { // Alt+Enter -> Split into 2 if's
        s.remove(0)
    }
}

// iter postfix completion
fn process(v: Vec<u8>) {
    v.iter // Ctrl+Space -> select `iter`
}

mod point {
    pub struct Point2D/*caret*/ { // Ctrl+N -> generate constructor
        x: i32,
        y: i32
    }
    pub struct Point3D {
        x: i32,
        y: i32,
        z: i32
    }
}