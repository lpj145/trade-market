use super::{channel::{self, Channel}, interested::Interested, stock::Stock};
use std::collections::HashMap;

pub struct Broker {
    pub intention_id: u32,
    stocks: HashMap<String, Stock>,
    channels: HashMap<String, Channel>,
    interested: Vec<Interested>
}

struct BrokerDetails {
    pub price: f32,
    pub stocks: u32,
    pub interested: u32,
    pub intentions: u32,
}

impl Broker {
    pub fn new () -> Self {
        Broker {
            intention_id: 0,
            stocks: HashMap::new(),
            channels: HashMap::new(),
            interested: Vec::new()
        }
    }

    pub fn add_channel(&mut self, channel: Channel) -> &mut Self {
        self.channels.insert(channel.name.to_string(), channel);
        self
    }

    // Add list of stocks to broker
    pub fn add_stock_list(&mut self, stocks: Vec<Stock>) -> &mut Self {
        for stock in stocks {
            self.stocks.insert(stock.name.to_string(), stock);
        }
        self
    }

    pub fn add_interested_list(&mut self, interested_list: Vec<Interested>) -> &mut Self {
        for interested in interested_list {
            self.interested.push(interested);
        }
        self
    }

    pub fn get_interested(&self) -> &Vec<Interested> {
        &self.interested
    }

    pub fn get_stocks(&self) -> &HashMap<String, Stock> {
        &self.stocks
    }

    pub fn get_next_intetion_id(&mut self) -> u32 {
        self.intention_id += 1;
        self.intention_id
    }
}