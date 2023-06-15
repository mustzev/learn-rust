mod first_word;
mod try_struct;

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello world");
    let word = first_word::first_word(&s);
    // s.clear();
    println!("word = {}", word);

    let user: try_struct::User = try_struct::try_struct();
    println!("user = {}", user.username);
}
