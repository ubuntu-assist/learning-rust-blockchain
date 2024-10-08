use crate::inventory::Product;
use crate::purchase::Purchase;
use crate::sales::Sale;
use prettytable::{cell, row, Table};
use std::collections::HashMap;

pub fn generate_inventory_report(inventory: &HashMap<u32, Product>) {
    let mut table = Table::new();
    table.add_row(row!["ID", "Name", "Description", "Price", "Quantity"]);

    for product in inventory.values() {
        table.add_row(row![
            product.id,
            product.name,
            product.description,
            format!("{:.2}", product.price),
            product.quantity,
        ]);
    }

    println!("--- Inventory Report ---");
    table.printstd();
}

pub fn generate_sales_report(sales: &Vec<Sale>) {
    let mut table = Table::new();
    table.add_row(row!["Product ID", "Quantity Sold", "Sale Price"]);

    for sale in sales {
        table.add_row(row![
            sale.product_id,
            sale.quantity_sold,
            format!("{:.2}", sale.sale_price),
        ]);
    }

    println!("--- Sales Report ---");
    table.printstd();
}

pub fn generate_purchase_report(purchases: &Vec<Purchase>) {
    let mut table = Table::new();
    table.add_row(row!["Product ID", "Quantity Purchased", "Purchase Price"]);

    for purchase in purchases {
        table.add_row(row![
            purchase.product_id,
            purchase.quantity_purchased,
            format!("{:.2}", purchase.purchase_price),
        ]);
    }

    println!("--- Purchase Report ---");
    table.printstd();
}
