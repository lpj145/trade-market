pub struct Wallet {
   money: f32
}

impl Wallet {
    pub fn new(money: f32) -> Self {
        Wallet {
            money
        }
    }
    pub fn can_buy(&self, target_price: f32) -> bool {
        self.money >= target_price
    }
}