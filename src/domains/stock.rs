pub struct Stock {
    pub name: String,
    pub price: f32,
    pub qty: u32
}

impl Stock {
    pub fn new(name: String, price: f32, qty: u32) -> Self {
        Stock {
            name,
            price,
            qty
        }
    }

    pub fn calculate_total_value(&self) -> f32 {
        self.price * self.qty as f32
    }
}
