mod lib;
use std::thread;
use std::time::Duration;
use closure::Cacher;

/*
    一个用来代替假定计算的函数
 */
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 27;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value,
                     simulated_random_number, );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {:?} pushups!", expensive_closure.value(intensity));
        println!("Next, do {:?} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {:?} minutes!", expensive_closure.value(intensity));
        }
    }
}

