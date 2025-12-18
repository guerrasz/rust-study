#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>,
}

impl ShoppingCart {
    fn trasverse_items<T>(&mut self, mut operation: T)
    where
        T: FnMut(&mut SupermarketItem),
    {
        for mut item in &mut self.items {
            operation(&mut item)
        }
    }

    fn checkout<T>(self, operation: T)
    where
        T: FnOnce(ShoppingCart),
    {
        operation(self)
    }
}

fn main() {
    let mut shopping_cart = ShoppingCart {
        items: vec![
            SupermarketItem {
                name: String::from("APPLE"),
                price: 3.99,
            },
            SupermarketItem {
                name: String::from("BANANA"),
                price: 1.99,
            },
        ],
    };

    shopping_cart.trasverse_items(|item: &mut SupermarketItem| item.price *= 0.85);

    shopping_cart
        .trasverse_items(|item: &mut SupermarketItem| item.name = item.name.to_uppercase());

    let mut total_price = 0.0;

    shopping_cart.checkout(|mut cart: ShoppingCart| {
        println!("{:?}", cart);
        cart.trasverse_items(|item: &mut SupermarketItem| total_price += item.price);
    });

    println!("Shopping cart price: ${:.2}", total_price)
}
