use super::broker::{Broker};
pub struct BrokerDetails {
    price: f32,
    stocks: u32,
    interested: u32,
    intentions: u32,
}

impl BrokerDetails {
    // Calculate all details about broker numbers
    pub fn new (broker: &Broker) -> Self {
        let mut details = BrokerDetails {
            price: 0.0,
            stocks: 0,
            intentions: broker.intention_id,
            interested: broker.get_interested().len() as u32,
        };

        for (_name, stock) in broker.get_stocks() {
            details.price += stock.calculate_total_value();
            details.stocks += stock.qty;
        }

        details
    }

    pub fn show_details(&self) {
        println!("{} opa", 0)
    }
}