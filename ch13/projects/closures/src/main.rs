use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    let intensity = simulated_expensive_calculation(5);
    println!("{}", intensity);

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_nunber: u32) {
    if intensity < 25 {
        println!(
            "tofay, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_nunber == 3 {
            println!("take a break today! remember to stay hydrated!");
        } else {
            println!(
                "today, run for {} minuites!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
