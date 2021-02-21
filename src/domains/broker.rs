use super::{display::Display, stock::Stock};
use std::collections::HashMap;

pub struct Broker {
    stocks: HashMap<String, Stock>,
    display: Display,
}

impl Broker {
    pub fn new () -> Self {
        Broker {
            stocks: HashMap::new(),
            display: Display::new()
        }
    }

    // Add list of stocks to broker
    pub fn add_stock_list(&mut self, stocks: Vec<Stock>) -> &mut Self {
        let stocks_qty = stocks.len();
        for stock in stocks {
            self.stocks.insert(stock.name.to_string(), stock);
        }
        self.display.message(format!("Registered {} stocks", stocks_qty));
        self
    }

    // Add one stock to broker
    pub fn add_stock(&mut self, stock: Stock) -> &mut Self {
        self.stocks.insert(stock.name.to_string(), stock);
        self
    }

    pub fn factory_stock(&self, name: String, price: f32, qty: u32) -> Stock {
        Stock {
            name,
            price,
            qty
        }
    }

    pub fn print_broker_details(&self) -> (){
        let mut total_stocks_price = 0.0;
        let mut total_stocks_qty: u32 = 0;
        for (_name, stock) in self.stocks.iter() {
            total_stocks_qty += stock.qty;
            total_stocks_price += stock.price;
        }

        self.display.message(format!("I haved {} registered stocks.", total_stocks_qty));
        self.display.message(format!("My total broker value is: {}", total_stocks_price));
        return;
    }
}