use std::collections::HashMap;

#[derive(Debug)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}

pub fn add_product(inventory: &mut HashMap<u32, Product>) {
    let mut name = String::new();
    let mut description = String::new();
    let mut price = String::new();
    let mut quantity = String::new();

    println!("Enter product name:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Enter product description:");
    std::io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");
    println!("Enter product price:");
    std::io::stdin()
        .read_line(&mut price)
        .expect("Failed to read input");
    println!("Enter product quantity:");
    std::io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");

    let id = inventory.len() as u32 + 1;
    let price: f64 = price.trim().parse().expect("Invalid price");
    let quantity: u32 = quantity.trim().parse().expect("Invalid quantity");

    let product = Product {
        id,
        name: name.trim().to_string(),
        description: description.trim().to_string(),
        price,
        quantity,
    };

    inventory.insert(id, product);
    println!("Product added successfully!");
}

pub fn edit_product(inventory: &mut HashMap<u32, Product>) {
    let mut product_id = String::new();
    println!("Enter product ID to edit:");
    std::io::stdin()
        .read_line(&mut product_id)
        .expect("Failed to read input");

    let product_id: u32 = product_id.trim().parse().expect("Invalid product ID");
    if let Some(product) = inventory.get_mut(&product_id) {
        let mut name = String::new();
        let mut description = String::new();
        let mut price = String::new();
        let mut quantity = String::new();

        println!("Enter new product name (leave blank to keep unchanged):");
        std::io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        println!("Enter new product description (leave blank to keep unchanged):");
        std::io::stdin()
            .read_line(&mut description)
            .expect("Failed to read input");
        println!("Enter new product price (leave blank to keep unchanged):");
        std::io::stdin()
            .read_line(&mut price)
            .expect("Failed to read input");
        println!("Enter new product quantity (leave blank to keep unchanged):");
        std::io::stdin()
            .read_line(&mut quantity)
            .expect("Failed to read input");

        if !name.trim().is_empty() {
            product.name = name.trim().to_string();
        }
        if !description.trim().is_empty() {
            product.description = description.trim().to_string();
        }
        if !price.trim().is_empty() {
            product.price = price.trim().parse().expect("Invalid price");
        }
        if !quantity.trim().is_empty() {
            product.quantity = quantity.trim().parse().expect("Invalid quantity");
        }

        println!("Product updated successfully!");
    } else {
        println!("Product not found!");
    }
}

pub fn delete_product(inventory: &mut HashMap<u32, Product>) {
    let mut product_id = String::new();
    println!("Enter product ID to delete:");
    std::io::stdin()
        .read_line(&mut product_id)
        .expect("Failed to read input");

    let product_id: u32 = product_id.trim().parse().expect("Invalid product ID");

    if inventory.remove(&product_id).is_some() {
        println!("Product deleted successfully!");
    } else {
        println!("Product not found!");
    }
}
