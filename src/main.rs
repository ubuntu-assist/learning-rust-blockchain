mod auth;
mod inventory;
mod purchase;
mod report;
mod sales;

use crate::auth::authenticate;
use crate::inventory::{add_product, delete_product, edit_product, Product};
use crate::purchase::{record_purchase, Purchase};
use crate::report::{generate_inventory_report, generate_purchase_report, generate_sales_report};
use crate::sales::{record_sale, Sale};
use std::collections::HashMap;

fn main() {
    let mut inventory: HashMap<u32, Product> = HashMap::new();
    let mut sales: Vec<Sale> = Vec::new();
    let mut purchases: Vec<Purchase> = Vec::new();

    // Authentication
    if !authenticate() {
        println!("Authentication failed. Exiting...");
        return;
    }

    loop {
        println!("--- Rusty Store Inventory Management ---");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Record Sale");
        println!("5. Record Purchase");
        println!("6. Generate Inventory Report");
        println!("7. Generate Sales Report");
        println!("8. Generate Purchase Report");
        println!("9. Exit");
        println!("Select an option:");

        let mut option = String::new();
        std::io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");
        let option: u32 = option.trim().parse().expect("Invalid option");

        match option {
            1 => add_product(&mut inventory),
            2 => edit_product(&mut inventory),
            3 => delete_product(&mut inventory),
            4 => record_sale(&mut inventory, &mut sales),
            5 => record_purchase(&mut inventory, &mut purchases),
            6 => generate_inventory_report(&inventory),
            7 => generate_sales_report(&sales),
            8 => generate_purchase_report(&purchases),
            9 => {
                println!("Exiting system...");
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}
