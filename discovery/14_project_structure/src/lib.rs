// import modules (declare the modules inside this lib)
pub mod inventory;
pub mod orders;

// use internal modules (shortcuts to syntax pieces)
pub use inventory::MANAGER as INV_MANAGER;
pub use inventory::products::{Item, ProductCategory};
pub use orders::MANAGER as ORD_MANAGER;
