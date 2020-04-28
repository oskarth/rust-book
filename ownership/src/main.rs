fn main() {
    let s = String::from("hello world");

    let first = first_word(&s[..]);

    println!("First word of '{}' is {}", s, first);

    let a = [1,2,3,4,5];

    let slice = &a[1..3];

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
