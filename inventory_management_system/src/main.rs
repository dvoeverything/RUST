use std::collections::HashMap;
use std::fmt;

/// ────────────────────────
/// Custom error type
/// ────────────────────────
#[derive(Debug)]
enum InventoryError {
    DuplicateItem { existing: Item },
    ItemNotFound { id: u32 },
    NameNotFound { name: String },
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateItem { existing } => write!(f, "item '{}' already exists", existing.name),
            Self::ItemNotFound { id }        => write!(f, "item with id {id} not found"),
            Self::NameNotFound { name }      => write!(f, "item named '{name}' not found"),
        }
    }
}

type InvResult<T> = Result<T, InventoryError>;

/// ────────────────────────
/// Data structures
/// ────────────────────────
#[derive(Debug, Clone)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
    price: f64,
}

#[derive(Debug, Default)]
struct Inventory {
    items: HashMap<u32, Item>,
    next_item_id: u32,
}

impl Inventory {
    fn new() -> Self {
        Self::default()
    }

    //  Add
    fn add_item(&mut self, name: String, quantity: u32, price: f64) -> InvResult<()> {
        // duplicate‐name check
        if let Some(existing) = self.items.values().find(|it| it.name == name) {
            return Err(InventoryError::DuplicateItem {
                existing: existing.clone(),
            });
        }

        // push new item
        let id = self.next_item_id;
        self.items.insert(
            id,
            Item {
                id,
                name,
                quantity,
                price,
            },
        );
        self.next_item_id += 1;
        Ok(())
    }

    // Update (partial)
    fn update_item(
        &mut self,
        id: u32,
        name: Option<String>,
        quantity: Option<u32>,
        price: Option<f64>,
    ) -> InvResult<()> {
        let item = self
            .items
            .get_mut(&id)
            .ok_or(InventoryError::ItemNotFound { id })?;

        if let Some(n) = name {
            item.name = n;
        }
        if let Some(q) = quantity {
            item.quantity = q;
        }
        if let Some(p) = price {
            item.price = p;
        }
        Ok(())
    }

    // Delete
    fn delete_item(&mut self, id: u32) -> InvResult<()> {
        if self.items.remove(&id).is_none() {
            return Err(InventoryError::ItemNotFound { id });
        }
        Ok(())
    }

    //  List (unchanged, never fails)
    fn list_items(&self) -> Vec<&Item> {
        self.items.values().collect()
    }

    //  Find by name
    fn find_item(&self, name: &str) -> InvResult<&Item> {
        self.items
            .values()
            .find(|it| it.name == name)
            .ok_or_else(|| InventoryError::NameNotFound { name: name.into() })
    }
}

/// ────────────────────────
/// Demo / manual test
/// ────────────────────────
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut inv = Inventory::new();

    inv.add_item("Laptop".into(), 10, 999.99)?;
    inv.add_item("Smartphone".into(), 20, 499.99)?;

    println!("── Inventory ──");
    for it in inv.list_items() {
        println!("{it:?}");
    }

    inv.update_item(1, Some("Gaming Laptop".into()), None, Some(1299.99))?;
    inv.delete_item(2)?;

    match inv.find_item("Gaming Laptop") {
        Ok(it) => println!("Found: {:?}", it),
        Err(e) => println!("Error: {e}"),
    }

    match inv.find_item("Business Laptop") {
        Ok(it) => println!("Found: {:?}", it),
        Err(e) => println!("Error: {e}"),
    }

    // Duplicate-name demo
    if let Err(e) = inv.add_item("Gaming Laptop".into(), 5, 879.99) {
        println!("Add failed: {e}");
    }

    Ok(())
}

/// ────────────────────────
/// Unit tests
/// ────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_item_ok() {
        let mut inv = Inventory::new();
        assert!(inv.add_item("Test".into(), 1, 1.0).is_ok());
    }

    #[test]
    fn add_item_duplicate() {
        let mut inv = Inventory::new();
        inv.add_item("Dup".into(), 1, 1.0).unwrap();
        match inv.add_item("Dup".into(), 2, 2.0) {
            Err(InventoryError::DuplicateItem { .. }) => {}
            _ => panic!("expected DuplicateItem error"),
        }
    }

    #[test]
    fn update_item_not_found() {
        let mut inv = Inventory::new();
        match inv.update_item(42, None, None, None) {
            Err(InventoryError::ItemNotFound { id }) if id == 42 => {}
            _ => panic!("expected ItemNotFound"),
        }
    }

    #[test]
    fn delete_item_ok_then_fail() {
        let mut inv = Inventory::new();
        inv.add_item("X".into(), 1, 1.0).unwrap();
        assert!(inv.delete_item(1).is_ok());
        assert!(matches!(
            inv.delete_item(1),
            Err(InventoryError::ItemNotFound { .. })
        ));
    }
}
