use crate::inventory::Product;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Purchase {
    pub product_id: u32,
    pub quantity_purchased: u32,
    pub purchase_price: f64,
}

pub fn record_purchase(inventory: &mut HashMap<u32, Product>, purchases: &mut Vec<Purchase>) {
    let mut product_id = String::new();
    let mut quantity = String::new();
    let mut purchase_price = String::new();

    println!("Enter product ID for purchase:");
    std::io::stdin()
        .read_line(&mut product_id)
        .expect("Failed to read input");
    println!("Enter quantity purchased:");
    std::io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");
    println!("Enter purchase price:");
    std::io::stdin()
        .read_line(&mut purchase_price)
        .expect("Failed to read input");

    let product_id: u32 = product_id.trim().parse().expect("Invalid product ID");
    let quantity: u32 = quantity.trim().parse().expect("Invalid quantity");
    let purchase_price: f64 = purchase_price
        .trim()
        .parse()
        .expect("Invalid purchase price");

    if let Some(product) = inventory.get_mut(&product_id) {
        product.quantity += quantity;
        purchases.push(Purchase {
            product_id,
            quantity_purchased: quantity,
            purchase_price,
        });
        println!("Purchase recorded successfully!");
    } else {
        println!("Product not found!");
    }
}
