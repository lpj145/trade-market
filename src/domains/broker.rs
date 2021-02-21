use super::{channel::Channel, intention::Intention, interested::Interested, stock::Stock};
use std::collections::HashMap;

pub struct Broker<'a> {
    pub intention_id: u32,
    stocks: HashMap<String, Stock>,
    channels: HashMap<String, Channel>,
    interested: Vec<&'a Interested>,
    intentions: Vec<Intention<'a>>
}

impl<'a> Broker<'a> {
    pub fn new () -> Self {
        Broker {
            intention_id: 0,
            stocks: HashMap::new(),
            channels: HashMap::new(),
            interested: Vec::new(),
            intentions: Vec::new(),
        }
    }

    pub fn add_channel(&mut self, channel: Channel) -> &mut Self {
        self.channels.insert(channel.name.to_string(), channel);
        self
    }

    pub fn add_intention(&mut self, intention: Intention<'a>) -> &mut Self {
        self.intentions.push(intention);
        self
    }

    // Add list of stocks to broker
    pub fn add_stock_list(&mut self, stocks: Vec<Stock>) -> &mut Self {
        for stock in stocks {
            self.stocks.insert(stock.name.to_string(), stock);
        }
        self
    }

    pub fn add_interested_list(&mut self, interested_list: Vec<&'a Interested>) -> &mut Self {
        for interested in interested_list {
            self.interested.push(interested);
        }
        self
    }

    pub fn get_interested(&self) -> &Vec<&'a Interested> {
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