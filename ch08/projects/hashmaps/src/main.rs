use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);
    for e in &scores {
        println!("{:?}", e);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores: {:?}", scores);
    for e in &scores {
        println!("{:?}", e);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);
    for e in &scores {
        println!("{:?}", e);
    }
    let team = String::from("Yellow");
    match scores.get(&team) {
        Some(score) => println!("{:?}", score),
        None => println!("not found"),
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        println!("{:?}", count);
        *count += 1;
    }
    println!("{:?}", map);
}
