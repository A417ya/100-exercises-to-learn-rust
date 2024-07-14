// TODO: Define a new `Order` type.
//X   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Order {
<<<<<<< HEAD
        assert!(product_name.is_empty(), "Product Name cannot be empty");
        assert!(
            product_name.len() <= 300,
            "Product Name is Bigger than 300 bytes"
        );
        assert!(
            quantity.is_positive(),
            "Quantity Name must be greater than zero"
        );
        assert!(
            unit_price.is_positive(),
            "Unit Price must be greater than zero"
        );
=======
        Self::_validate_product_name(&product_name);
        Self::_validate_quantity(&quantity);
        Self::_validate_unit_price(&unit_price);
>>>>>>> b9b0715

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

<<<<<<< HEAD
    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }
=======
    fn _validate_product_name(product_name: &String) {
        assert!(!product_name.is_empty(), "Product Name cannot be empty");
        assert!(
            product_name.len() <= 300,
            "Product Cannot be bigger than 300 bytes"
        )
    }

    pub fn set_product_name(&mut self, new_product_name: String) {
        Self::_validate_product_name(&new_product_name);
        self.product_name = new_product_name;
    }

    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }

    fn _validate_unit_price(unit_price: &i32) {
        assert!(
            unit_price.is_positive(),
            "Unit Price must be greater than zero"
        );
    }

    pub fn set_unit_price(&mut self, new_unit_price: i32) {
        Self::_validate_unit_price(&new_unit_price);
        self.unit_price = new_unit_price;
    }

    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }

    fn _validate_quantity(quantity: &i32) {
        assert!(
            quantity.is_positive(),
            "Quantity Name must be greater than zero"
        );
    }

    pub fn set_quantity(&mut self, new_quantity: i32) {
        Self::_validate_quantity(&new_quantity);
        self.quantity = new_quantity;
    }

    pub fn total(&self) -> i32 {
        &self.quantity * &self.unit_price
    }
>>>>>>> b9b0715
}
