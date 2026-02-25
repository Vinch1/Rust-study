#![allow(warnings)]

use std::result;

fn main() {
    hello_word();
    // let e:i32 = 100;
    let is_snowing: bool = true;
    println!("is snowing? {}", is_snowing);
    // let letter: char = 'a';
    let letterstring: &str = "aaaaaa";
    println!("letter string is: {}{}", letterstring, "ddfddf");
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("the numbers are: {:?}", numbers);

    // Arrays
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("the fruits are: {:?}", fruits);

    let fruits: Vec<&str> = vec!["apple", "banana", "orange"];
    println!("the fruits are: {:?}", fruits);

    // Tuples
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("the person is: {:?}", person);
    let my_mix_tuple = ("Kratos", 30, true, [1, 2, 3, 4, 5]);
    println!("the mix tuple is: {:?}", my_mix_tuple);

    // Slices contagiously in memory
    let number_slices: &[i32; 5] = &[1, 2, 3, 4, 5];
    println!("the number slices are: {:?}", number_slices);
    let book_slices: &[&String] = &[
        &"The Rust Programming Language".to_string(),
        &"Programming Rust".to_string(),
    ];
    println!("Book Slices: {:?}", book_slices);

    
    // Strings Vs String slices &str
    let mut stone_cold: String = String::from("hell");
    stone_cold.push_str(" yeah! on heap");
    println!("stone cold says: {}", stone_cold);
    
    // &str in stack, reference
    let string: String = String::from("HEllo, world!");
    let slice: &str = &string[0..5];
    println!("string: {}, slice: {}", string, slice);
    tell_height(25);
    human_id("Leo Jiang", 22);
    let _X = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("The total is: {}", _X);
    let y = add(4, 6);
    println!("The sum of y is: {}", y);
    //call BMI fn
    println!("The BMI is: {:.2}", calculate_bmi(79.0, 1.82));
    
    // Ownership and borrowing
    let s1 = String::from("Rust");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // let s2 = s1; // transfer ownership of s1 to s2
    // use '&' to borrow
    let mut x = 5;
    {
        let y = &mut x; // borrow x
        *y += 1;
        println!("The value of y is: {}", y);
    } // y goes out of scope here, mutable borrow ends
    println!("The value of x is: {}", x);
    
    let mut account = BankAccount {
        account_number: "123456789".to_string(),
        balance: 5000.0,
    };
    // immutable to Borrowing the account to check balance
    account.check_balance();
    // mutable to Borrowing the account to withdraw money
    account.withdraw(690.69);
    account.check_balance();
    
    // Constants
    const Y: u32 = 100;
    println!("The value of Y is: {}", Y);
    println!("The value of PI is: {}", PI);
    
    // Shadowing
    let x = 5;
    let x = x + 1; // shadows the previous x
    {
        let x = x * 2; // shadows the previous x again
        println!("The value of shadowed x in inner scope is: {}", x);
    }
    println!("The value of shadowed x is: {}", x);
    
    //control flow
    let age: u32 = 18;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }
    let number = 7;
    if number % 4 == 0 {
        println!("{} is divisible by 4.", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2 but not by 4.", number);
    } else {
        println!("{} is not divisible by 2 or 4.", number);
    }
    
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("number is: {}", number);
    
    // Loops
    // loop{
    // }
    //     println!("This will loop forever!");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // return value from loop
        }
    };
    println!("The result from the loop is: {result}");
    
    let mut count = 0;
    'counting_up: loop { // label
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {count}");
    
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the element is: {element}");
    }

    //structs
    let mut user1: User = User{
        active: true,
        username: String::from("leojiang"),
        email: String::from("leojiang@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("leo@gmail.com");

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    let user2 = User{
        email: String::from("another.com"),
        ..user1
    };
    //tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // enum
}

struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}
    
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
    
    
    
const PI: f64 = 3.14159;

struct BankAccount {
    account_number: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        // takes a mutable reference to self not Ownership
        println!(
            "Withdrawing ${} from account {}",
            amount, self.account_number
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "The balance for account {} is ${}",
            self.account_number, self.balance
        );
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn hello_word() {
    println!("Hello, Crab!");
}

fn tell_height(height: i32) {
    println!("the height is: {}", height);
}

fn human_id(name: &str, age: u32) -> String {
    format!("My name is {} and I am {} years old.", name, age)
}
