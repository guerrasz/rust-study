// import modules (declare the modules inside this lib)

/// Tools for inventory management
pub mod inventory;

/// Tools for order management
pub mod orders;

// use internal modules (shortcuts to syntax pieces)
pub use inventory::MANAGER as INV_MANAGER;
pub use inventory::products::{Item, ProductCategory};
pub use orders::MANAGER as ORD_MANAGER;
