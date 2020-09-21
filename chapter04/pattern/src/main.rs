pub struct DemoStruct {
    pub id: u64,
    pub name: String,
    pub probability: f64,
}

pub fn might_fail(id: u64) -> Result<DemoStruct, &'static str> {
    if id % 2 == 0 {
 Ok(DemoStruct { id: id, name: String::from("An Even Thing"), probability: 0.2})
 }
 else {
 Err("Only even numbers are allowed")
 }
}

fn main() {
    println!("Hello, world!");

    let source2 = DemoStruct { id: 35, name: String::from("Another Thing"), probability: 0.42 };
    let source3 = DemoStruct { id: 63, name: String::from("Super Thing"), probability: 0.99 };
    if let DemoStruct { id: 63, name: y, probability: z } = source2 {
        println!("When id is 63, name is {} and probability is {}", y, z);
    }
    
    if let DemoStruct { id: 63, name: y, probability: z } = source3 {
        println!("When id is 63, name is {} and probability is {}", y, z);
    }

    if let Ok(x) = might_fail(37) {
        println!("Odd succeeded, name is {}", x.name);
    }

    if let Ok(x) = might_fail(38) {
        println!("Even succeeded, name is {}", x.name);
    }
    else if let Err(x) = might_fail(39) {
        println!("Odd failed, message is '{}'", x);
    }

    match might_fail(38) {
        Ok(x) => { println!("Odd succeeded, name is {}", x.name) }
        Err(x) => { println!("Odd failed, message is '{}'", x) }
    }
    match might_fail(39) {
        Ok(x) => { println!("Odd succeeded, name is {}", x.name) }
        Err(x) => { println!("Odd failed, message is '{}'", x) }
    }
}

