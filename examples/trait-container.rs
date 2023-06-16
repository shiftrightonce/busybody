#![allow(dead_code)]

use busybody::*;
use rand::Rng;
use std::fmt::Debug;
use std::ops::Deref;

fn main() {
    // 1. Setup the container by using the service builder
    let container = ServiceContainerBuilder::new().build();

    let mut rng = rand::thread_rng(); // for random numbers generation

    // 2. Inject a concrete implementation of a trait.
    //    In this case we are selecting an implementation randomly
    if rng.gen_range(2..17) % 2 == 0 {
        container.set(AdderProvider::new(MyAdder1 { id: 2000 }));
    } else {
        container.set(AdderProvider::new(MyAdder2));
    }

    // 3. Get the AdderProvider
    let foo = container.get::<AdderProvider>().unwrap();

    let number1: i32 = rng.gen_range(0..200);
    let number2: i32 = rng.gen_range(3..100);
    println!(
        "sum of {} + {} = {}",
        number1,
        number2,
        foo.add(number1, number2)
    );
}

trait Add: Debug + Send + Sync {
    fn add(&self, num1: i32, num2: i32) -> i32;
}

#[derive(Debug, Clone)]
struct MyAdder1 {
    id: i32,
}

impl Add for MyAdder1 {
    fn add(&self, num1: i32, num2: i32) -> i32 {
        println!("Using MyAdder1");
        num1 + num2
    }
}

#[derive(Debug, Clone)]
struct MyAdder2;

impl Add for MyAdder2 {
    fn add(&self, num1: i32, num2: i32) -> i32 {
        println!("Using MyAdder2");
        num1 + num2 * 2
    }
}

#[derive(Debug)]
struct AdderProvider(Box<dyn Add>);

impl AdderProvider {
    pub fn new<T: Add + 'static>(adder: T) -> Self {
        Self(Box::new(adder))
    }
}

// Makes `AdderProvider` appears as `Add`
impl Deref for AdderProvider {
    type Target = Box<dyn Add>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
