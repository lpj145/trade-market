use super::wallet::Wallet;
pub struct Interested {
    pub name: String,
    pub wallet: Wallet
}

impl Interested {
    pub fn new(name: String, available_money: f32) -> Self {
        Interested {
            name,
            wallet: Wallet::new(available_money)
        }
    }
}