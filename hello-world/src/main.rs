use std::io::Write;

#[allow(static_mut_refs)]
fn input(prompt: &str) -> String {
    static mut COUNTER: u32 = 0;

    unsafe {
        COUNTER += 1;
        println!("Function input() called {} times", COUNTER);
    }

    print!("{}: ", prompt);
    std::io::stdout().flush().unwrap();

    let mut value: String = String::new();
    std::io::stdin().read_line(&mut value).unwrap(); 
    value.trim().to_string()
}

mod structs {
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    pub fn create_person(name: &str, age: u8) -> Person {
        let mut p = Person {
            name: name.to_string(),
            age,
        };

        p.age += 1; // Happy birthday!
        p
    }
}


fn main() {
    let person = crate::structs::create_person("Bob", 25);

    println!("Name: {}, Age: {}", person.name, person.age);


    let big_number = std::i32::MAX;
    
    //big_number += 1;
    let big_result = big_number.checked_add(1);

    match big_result {
        Some(r) => println!("result: {}", r),
        None => println!("Arthemtic overflow detected...")
    };

    let greeting: &str = "Hello, ";
    
    let name: String = input("Get name");
    println!("{}{}!", greeting, name);

    let age_str: String = input("Get your age");

    match age_str.parse::<u32>() {
        Ok(age) => println!("You are {age} years old."),
        Err(_) => println!("Invalid age input.")
    }
}
