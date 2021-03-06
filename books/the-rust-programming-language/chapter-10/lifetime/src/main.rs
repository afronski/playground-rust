fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("ABCDEF");
    let b = String::from("ABCDE");

    println!("Which is longest? {} vs {} = {}", a, b, longest(a.as_str(), b.as_str()));
}
