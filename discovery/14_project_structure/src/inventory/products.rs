use fake::Dummy;

/// Category of products used in bussiness
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

/// Item construct containing needed properties
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
}

impl Item {
    /// Creates a new item instance
    pub fn new(name: String, category: ProductCategory) -> Item {
        super::talk_manager();
        Item { name, category }
    }
}
