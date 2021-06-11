use std::collections::HashMap;
use std::thread;
#[allow(unused_imports)]
use std::time::{Duration, SystemTime};

// legacy
// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|intensity |  {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!("Today do {} pushups", expensive_result.value(intensity));
        println!("Next do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! remember to stay hydrated!");
        } else {
            println!(
                "Today run for {} minutes",
                expensive_result.value(intensity)
            )
        }
    }
}

struct Cacher<T, R, L>
where
    T: Fn(R) -> L,
{
    calculation: T,
    value: HashMap<R, L>,
}

impl<T, R, L> Cacher<T, R, L>
where
    T: Fn(R) -> L,
    R: Eq + Clone + std::hash::Hash,
    L: Eq + Copy,
{
    fn new(calculation: T) -> Cacher<T, R, L> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: R) -> L {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg.clone());
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        assert_eq!(v1, 1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_string() {
        let mut c = Cacher::new(|a| a);
        let foo = String::from("foo");
        let v1 = c.value(&foo);
        assert_eq!(v1, "foo");
    }
}

