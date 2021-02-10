fn main() {
    let mut s = String::from("hello");

    println!("PRE: {}", s);

    change(&mut s);

    println!("POST: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
