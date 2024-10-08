// fn main() {
//     let string1 = String::from("Rust is a ");
//     let string2 = String::from("memory safety programming language");

//     let concatenated_string = concatenate_strings(&string1, &string2);
//     println!("The resulting string is: {}", concatenated_string);

//     let book = Book {
//         title: String::from("Programming Rust"),
//         author: String::from("The Rust Programming Language"),
//         publication_year: 2018,
//     };

//     println!(
//         "The book {}, written by {} was published in {}",
//         book.title, book.author, book.publication_year
//     );

//     let book_data = get_book_data(book);
//     for data in book_data {
//         println!("{data}");
//     }

//     let new_book = create_book(String::from("TalantaApp"), String::from("Duclair"), 2024);
//     println!("New book: {:?}", new_book);

//     let tuple_book = Tuple_Book("Ubuntu Assist".to_string(), "Fouwa".to_string(), 2023);
//     let title = tuple_book.0;

//     let rectangle = Rectangle {
//         width: 5.0,
//         height: 3.0,
//     };

//     let area = rectangle.area();
//     println!("The area of the rectangle is: {}", area);

//     let current_weather = Weather::Rainy;
//     let msg = Message::Move { x: 5, y: 5 };
//     process_message(msg);

//     let pet = Animal::Bird("Fouwa".to_string());

//     if let Animal::Bird(name) = pet {
//         println!("The animal is a bird named {}.", name);
//     } else {
//         println!("The animal is not a bird.");
//     }

//     let res = square_root(-40.5);

//     match res {
//         Some(value) => println!("The square root of -40.5 is {}", value),
//         None => println!("The square root of -40.5 is a complex number"),
//     }

//     let a = 5.6;
//     let b = 0.0;

//     let quotient = divide(a, b);

//     match quotient {
//         Ok(value) => println!("The division result is {value}"),
//         Err(error) => println!("Error: {error}"),
//     }

//     let base = get_from_database("base");
//     let height = get_from_database("height");
//     let area_value = calculate_triangle_area(base, height);

//     match area_value {
//         Ok(value) => println!("The area result is {value}"),
//         Err(error_msg) => println!("Error: {error_msg}"),
//     }
// }

// fn concatenate_strings(s1: &String, s2: &String) -> String {
//     let mut result = String::new();
//     result.push_str(s1);
//     result.push_str(s2);
//     result
// }

// #[derive(Debug)]
// struct Book {
//     title: String,
//     author: String,
//     publication_year: u32,
// }

// struct Tuple_Book(String, String, u32);

// fn get_book_data(book: Book) -> [String; 3] {
//     let title = book.title;
//     let author = book.author;
//     let publication_year = book.publication_year.to_string();

//     let data: [String; 3] = [title, author, publication_year];
//     data
// }

// fn create_book(title: String, author: String, publication_year: u32) -> Book {
//     Book {
//         title,
//         author,
//         publication_year,
//     }
// }

// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// impl Rectangle {
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }
// }

// // enums
// enum Weather {
//     Sunny,
//     Cloudy,
//     Rainy,
//     Snowy,
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(u32, u32, u32),
// }

// impl Message {
//     fn call(&self) {
//         match self {
//             Message::Quit => println!("Quiting"),
//             Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
//             Message::Write(text) => println!("Writing: {}", text),
//             Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
//         }
//     }
// }

// fn process_message(msg: Message) {
//     match msg {
//         Message::Quit => println!("Quiting"),
//         Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
//         Message::Write(text) => println!("Writing: {}", text),
//         Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
//     }
// }

// enum Animal {
//     Dog(String),
//     Cat(String),
//     Bird(String),
// }

// fn square_root(number: f64) -> Option<f64> {
//     if number < 0.0 {
//         None
//     } else {
//         Some(number.sqrt())
//     }
// }

// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 {
//         Err(String::from("Division by zero not allowed"))
//     } else {
//         Ok(a / b)
//     }
// }

// fn get_from_database(key: &str) -> Option<f64> {
//     let database: [(&str, Option<f64>); 2] = [("base", Some(6.0)), ("height", Some(5.6))];

//     for (k, v) in database {
//         if k == key {
//             return v;
//         }
//     }

//     None
// }

// fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
//     match (base, height) {
//         (Some(b), Some(h)) => {
//             if h <= 0.0 || b <= 0.0 {
//                 Err(String::from("Invalid base or height"))
//             } else {
//                 Ok(0.5 * b * h)
//             }
//         }
//         (None, _) => Err(String::from("The base is missing")),
//         (_, None) => Err(String::from("The height is missing")),
//     }
// }

// use std::io;

// fn main() {
// let num1 = get_input("Enter the first number:");
// let op = get_operation();
// let num2 = get_input("Enter the second number:");

// let operation = match op.as_str() {
//     "+" => Operation::Add(num1, num2),
//     "-" => Operation::Subtract(num1, num2),
//     "*" => Operation::Multiply(num1, num2),
//     "/" => Operation::Divide(num1, num2),
//     _ => return println!("Invalid operation. Please use +, -, *, or /."),
// };

// match calculate(operation) {
//     Ok(result) => println!("Result: {}", result),
//     Err(error) => println!("{}", error),
// }

// let dog = Dog {
//     name: String::from("Rex"),
// };

// dog.speak();

// let cow = Cow {
//     name: String::from("Moo"),
// };

// cow.speak();
// }

// enum Operation {
//     Add(f64, f64),
//     Subtract(f64, f64),
//     Multiply(f64, f64),
//     Divide(f64, f64),
// }

// fn calculate(operator: Operation) -> Result<f64, String> {
//     match operator {
//         Operation::Add(n1, n2) => Ok(n1 + n2),
//         Operation::Subtract(n1, n2) => Ok(n1 - n2),
//         Operation::Multiply(n1, n2) => Ok(n1 * n2),
//         Operation::Divide(n1, n2) => {
//             if n2 == 0.0 {
//                 Err("Division by zero is not allowed".to_string())
//             } else {
//                 Ok(n1 / n2)
//             }
//         }
//     }
// }

// fn get_input(prompt: &str) -> f64 {
//     let mut input = String::new();
//     println!("{}", prompt);
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read input");
//     input.trim().parse().expect("Please enter a valid number")
// }

// fn get_operation() -> String {
//     let mut op_input = String::new();
//     println!("Enter the operation (+, -, *, /):");
//     io::stdin()
//         .read_line(&mut op_input)
//         .expect("Failed to read input");
//     op_input.trim().to_string()
// }

// trait Speak {
//     fn speak(&self);
// }

// struct Dog {
//     name: String,
// }

// struct Cow {
//     name: String,
// }

// impl Speak for Dog {
//     fn speak(&self) {
//         println!("{} says woof!", self.name);
//     }
// }

// impl Speak for Cow {
//     fn speak(&self) {
//         println!("{} says moo!", self.name);
//     }
// }

// Define the Account trait with deposit, withdraw, and balance methods
// trait Account {
//     fn deposit(&mut self, amount: f64);
//     fn withdraw(&mut self, amount: f64);
//     fn balance(&self) -> f64;
// }

// // Define a struct called BankAccount with account_number, holder_name, and balance fields
// struct BankAccount {
//     account_number: u64,
//     holder_name: String,
//     balance: f64,
// }

// // Implement the Account trait for the BankAccount struct
// impl Account for BankAccount {
//     fn deposit(&mut self, amount: f64) {
//         self.balance += amount;
//     }

//     fn withdraw(&mut self, amount: f64) {
//         if self.balance >= amount {
//             self.balance -= amount;
//         } else {
//             println!("Insufficient funds!");
//         }
//     }

//     fn balance(&self) -> f64 {
//         self.balance
//     }
// }

// fn main() {
//     // Create two BankAccount instances
//     let mut account1 = BankAccount {
//         account_number: 123456789,
//         holder_name: String::from("Alice"),
//         balance: 1000.0,
//     };

//     let mut account2 = BankAccount {
//         account_number: 987654321,
//         holder_name: String::from("Bob"),
//         balance: 500.0,
//     };

//     // Deposit money into account1
//     account1.deposit(200.0);
//     println!(
//         "{}'s new balance after deposit: ${}",
//         account1.holder_name,
//         account1.balance()
//     );

//     // Withdraw money from account2
//     account2.withdraw(150.0);
//     println!(
//         "{}'s new balance after withdrawal: ${}",
//         account2.holder_name,
//         account2.balance()
//     );

//     // Check the balance of both accounts
//     println!(
//         "{}'s final balance: ${}",
//         account1.holder_name,
//         account1.balance()
//     );
//     println!(
//         "{}'s final balance: ${}",
//         account2.holder_name,
//         account2.balance()
//     );
// }

// Define the FilterCondition struct
struct FilterCondition {
    value: i32, // Assuming we're filtering based on an integer value
}

// Implement methods for the FilterCondition struct
impl FilterCondition {
    // Check if the item matches the filter condition
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value // Match if the item equals the condition value
    }
}

// Custom filtering function
fn custom_filter(collection: Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered = Vec::new(); // Create a new vector to store filtered results

    // Iterate over the collection and check for matches
    for item in collection {
        if condition.is_match(&item) {
            filtered.push(item); // Add item to the filtered vector if it matches
        }
    }

    filtered // Return the filtered collection
}

fn main() {
    // Create a collection of integers
    let collection = vec![1, 2, 3, 4, 5, 3, 2, 1];

    // Initialize a FilterCondition object with the desired value
    let condition = FilterCondition { value: 3 };

    // Call the custom_filter function
    let filtered_result = custom_filter(collection, &condition);

    // Print the filtered result
    println!("Filtered result: {:?}", filtered_result);
}
