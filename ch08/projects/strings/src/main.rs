fn main() {
    let mut s = String::new();
    println!("string: {}", s);

    let s = "initial contents".to_string();
    println!("string: {}", s);

    let hello = String::from("Dobrý den");
    println!("hello: {}", hello);
    let hello = String::from("Hello");
    println!("hello: {}", hello);
    let hello = String::from("नमस्ते");
    println!("hello: {}", hello);
    let hello = String::from("こんにちは");
    println!("hello: {}", hello);
    let hello = String::from("안녕하세요");
    println!("hello: {}", hello);
    let hello = String::from("你好");
    println!("hello: {}", hello);
    let hello = String::from("Olá");
    println!("hello: {}", hello);
    let hello = String::from("Здравствуйте");
    println!("hello: {}", hello);
    let hello = String::from("Hola");
    println!("hello: {}", hello);

    let mut s = String::from("foo");
    let t = String::from("bar");
    s.push_str(&t);
    println!("string: {}", t);

    let s = String::from("Hello, ");
    let t = String::from("world!");
    let u = s + &t;
    println!("string: {}", u);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
