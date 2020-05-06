mod first;
mod forth;
mod lifetime;
mod read;
mod second;

fn main() {
    println!("Hello");
    read::read_file("hello.txt");
}
