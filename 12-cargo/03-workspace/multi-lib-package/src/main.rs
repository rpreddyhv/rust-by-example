use {lib_one, lib_two};

fn main() {
    println!("lib_one: {}", lib_one::add(1, 1));
    println!("lib_two: {}", lib_two::add(2, 2));
    println!("Hello, world!");
}
