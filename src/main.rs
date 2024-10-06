fn main() {
  let string1 = String::from("Rust is a ");
  let string2 = String::from("memory safety programming language");

  let concatenated_string = concatenate_strings(&string1, &string2);
  println!("The resulting string is: {}", concatenated_string);
}

fn concatenate_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}
