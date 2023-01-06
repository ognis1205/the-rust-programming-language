use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i64> = Vec::new();
    let len = rng.gen_range(5..21);

    for _ in 0..len {
        numbers.push(rng.gen_range(-100..101));
    }
    println!("numbers: {:?}", numbers);
    println!("largest: {:?}", find2(&numbers));
}

fn find(xs: &[i64]) -> i64 {
    let mut ret = xs[0];
    for &x in xs.iter() {
        if x > ret {
            ret = x;
        }
    }
    ret
}

fn find2<T: PartialOrd + Copy>(xs: &[T]) -> T {
    let mut ret = xs[0];
    for &x in xs.iter() {
        if x > ret {
            ret = x;
        }
    }
    ret
}
