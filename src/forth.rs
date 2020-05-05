struct User {
    name: String,
}

fn main() {
    let name = String::from("User");

    let user1 = User { name };
    //let user2 = User { name }; // compile error
}

fn get_string() -> String {
    let string = String::from("hello");

    drop(string);

    //return string; // compile error!
    return String::from("hello");
}

use std::thread;

fn threads() {
    let mut list = vec![];

    let f = move || {
        for _ in 0..10000 {
            list.push("item 123");
        }
    };

    let t1 = thread::spawn(f);
    //let t2 = thread::spawn(f); // compile error!

    t1.join().unwrap();
    //t2.join().unwrap();
}

//Usually, &str will be used for function parameters and String will be used as return value.
fn repeat(s: &str, count: u32) -> String {
    let mut result = String::new();
    for _ in 0..count {
        result += s;
    }
    result
}

fn vec() {
    let mut arr1 = vec![];
    arr1.push(1);
    arr1.push(2);
}
