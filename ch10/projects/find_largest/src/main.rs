use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i64> = Vec::new();
    let len = rng.gen_range(5..21);

    for _ in 0..len {
        numbers.push(rng.gen_range(-100..101));
    }
    println!("numbers: {:?}", numbers);

    let mut largest = numbers[0];
    for x in numbers {
        if x > largest {
            largest = x;
        }
    }
    println!("largest: {:?}", largest);
}
