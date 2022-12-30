fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("first word: {}", word);

    let s = String::from("hello world");
    let word = first_word_2(&s);
    println!("first word: {}", word);

    let s = String::from("hello world, this is me");
    let nth = 3;
    let word = nth_word(&s, nth);
    println!("{} word: {}", nth, word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn nth_word(s: &String, mut nth: usize) -> &str {
    let mut prev: usize = 0;
    let bytes = s.trim().as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            nth -= 1;
            if nth == 0 {
                return &s[prev..i];
            } else {
                prev = i + 1;
            }
        }
    }

    &s[..]
}
