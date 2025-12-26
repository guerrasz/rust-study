#[derive(Debug)]
struct Museum {
    paitings: Vec<String>,
    revenue: u32,
}

impl Museum {
    fn new() -> Self {
        Self {
            paitings: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        self.paitings.push(painting.to_string());
    }

    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    fn has_impressive_collection(&self) -> bool {
        self.paitings.len() > 2
    }
}

#[test]
fn musuem_sells_ticket_to_increase_revenue() {
    let mut museum = Museum::new();
    museum.sell_ticket();
    assert_eq!(museum.revenue, 25);
}
