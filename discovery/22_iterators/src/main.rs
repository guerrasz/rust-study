#![allow(unused, dead_code)]

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order: [u32; 6] = [2, 1, 2, 3, 4, 1];

    find_blender(&orders);

    get_total_microwaves(&orders);

    get_from_qnt(
        std::env::args()
            .nth(1)
            .unwrap_or(String::from("2"))
            .parse()
            .unwrap_or(2),
        &orders,
    );

    get_unshipped(&orders);

    ship_first_order(&mut orders);
    println!("Now Blender is shipped:\n {:#?}", orders);

    let mut grouped = create_customer_vec(&customer_ids_by_order, orders);
    sort_customer_vec(&mut grouped);
    println!("{:#?}", grouped)
}

fn find_blender(order_vec: &[CustomerOrder]) {
    let blenders: Vec<&CustomerOrder> = order_vec
        .iter()
        .filter(|order| order.product == Product::Blender)
        .collect();
    println!("Blenders: {:#?}\n", blenders)
}

fn get_total_microwaves(order_vec: &[CustomerOrder]) {
    let total_microwaves = order_vec
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .reduce(|total, current| total + current)
        .unwrap_or(0);

    println!("Total microwaves: {}\n", total_microwaves)
}

fn get_from_qnt(qnt: u32, orders: &[CustomerOrder]) {
    let filtered_orders: Vec<&CustomerOrder> = orders
        .iter()
        .filter(|order| order.quantity >= qnt)
        .collect();

    println!("Orders with quantity >= {} : {:#?}\n", qnt, filtered_orders)
}

fn get_unshipped(orders: &[CustomerOrder]) {
    let mut unshipped = HashMap::new();

    for order in orders {
        if !order.shipped {
            let entry = unshipped.entry(&order.product).or_insert(0);
            *entry += order.quantity;
        }
    }

    println!("Unshipped orders inventory: {:#?}\n", unshipped);
}

fn ship_first_order(orders: &mut [CustomerOrder]) -> &mut [CustomerOrder] {
    match orders.into_iter().find(|order| !order.shipped) {
        Some(to_ship) => to_ship.shipped = true,
        None => println!("All orders are shipped"),
    };

    orders
}

fn create_customer_vec(customer_ids: &[u32], orders: Vec<CustomerOrder>) -> Vec<Customer> {
    let mut customers: Vec<Customer> = Vec::new();
    for (id, order) in customer_ids.iter().zip(orders) {
        // seach for exisitng customer.id if existing push order to struct else create new struct
        match customers.iter_mut().find(|c| c.id == *id) {
            Some(cus) => cus.orders.push(order),
            None => customers.push(Customer {
                id: *id,
                orders: vec![order],
            }),
        }
    }

    customers
}

fn sort_customer_vec(customers: &mut [Customer]) {
    customers.sort_by(|a, b| a.id.cmp(&b.id));
}
