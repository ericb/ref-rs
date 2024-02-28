use gaze::foo;
mod rusqlite {
    pub mod rusqlite;
}

fn main() {
    println!("Hello, world!");
    println!("bar = {}", foo::bar());

    // Call the rusqlite module
    // This will create table then get data from the table
    let _ = rusqlite::rusqlite::main(); // Ignore the Result
}
