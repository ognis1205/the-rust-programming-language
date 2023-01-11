use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_nunber: u32) {
    let mut result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num * 2
    });

    if intensity < 25 {
        println!("tofay, do {} pushups!", result.value(intensity));
        println!("tofay, do {} pushups agian!", result.value(intensity + 1));
        println!("next, do {} situps!", result.value(intensity));
    } else {
        if random_nunber == 3 {
            println!("take a break today! remember to stay hydrated!");
        } else {
            println!("today, run for {} minuites!", result.value(intensity + 1));
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash,
{
    calculation: T,
    values: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}
