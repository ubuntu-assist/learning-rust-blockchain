use crate::inventory::Product;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Sale {
    pub product_id: u32,
    pub quantity_sold: u32,
    pub sale_price: f64,
}

pub fn record_sale(inventory: &mut HashMap<u32, Product>, sales: &mut Vec<Sale>) {
    let mut product_id = String::new();
    let mut quantity = String::new();
    let mut sale_price = String::new();

    println!("Enter product ID for sale:");
    std::io::stdin()
        .read_line(&mut product_id)
        .expect("Failed to read input");
    println!("Enter quantity sold:");
    std::io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");
    println!("Enter sale price:");
    std::io::stdin()
        .read_line(&mut sale_price)
        .expect("Failed to read input");

    let product_id: u32 = product_id.trim().parse().expect("Invalid product ID");
    let quantity: u32 = quantity.trim().parse().expect("Invalid quantity");
    let sale_price: f64 = sale_price.trim().parse().expect("Invalid sale price");

    if let Some(product) = inventory.get_mut(&product_id) {
        if product.quantity >= quantity {
            product.quantity -= quantity;
            sales.push(Sale {
                product_id,
                quantity_sold: quantity,
                sale_price,
            });
            println!("Sale recorded successfully!");
        } else {
            println!("Not enough stock!");
        }
    } else {
        println!("Product not found!");
    }
}
