use std::io::stdin;

// Unresolved method
fn main() {
    let mut buf = Vec::new();
    stdin().read/*caret*/(&mut buf); // pres Alt + Enter to import `Read` trait
}

// Elide lifetimes
fn process<'a>(s: &'a str) { // Alt + Enter -> Elide lifetimes

}

// Move analysis
pub struct Package {
    name: String,
    version: String
}

fn foo(pkg: Package) {
    consume(pkg.name);
    let version = pkg.name; // Error annotation here
}

fn consume(pkg: String) {}


