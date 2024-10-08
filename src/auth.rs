pub fn authenticate() -> bool {
    let username = "admin";
    let password = "password";

    let mut input_username = String::new();
    let mut input_password = String::new();

    println!("Enter username:");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read input");
    println!("Enter password:");
    std::io::stdin()
        .read_line(&mut input_password)
        .expect("Failed to read input");

    if input_username.trim() == username && input_password.trim() == password {
        println!("Authentication successful!");
        true
    } else {
        println!("Authentication failed!");
        false
    }
}
