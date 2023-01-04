fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("vector: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12),
    ];
    for c in &row {
        match c {
            Cell::Int(i) => println!("int: {}", i),
            Cell::Float(f) => println!("float: {}", f),
            Cell::Text(s) => println!("text: {}", s),
        }
    }
}
