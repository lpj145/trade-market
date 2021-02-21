mod domains;

use domains::{
    broker::Broker,
    stock::Stock,
    interested::Interested,
    broker_details::BrokerDetails
};

fn main() {
    let mut broker = Broker::new();
    broker
        .add_stock_list(vec![
            Stock::new("MGLU3".to_string(), 24.94, 500),
            Stock::new("PCAR3".to_string(), 64.75, 200),
            Stock::new("BIDI11".to_string(), 159.14, 3000),
            Stock::new("NTCO3".to_string(), 52.55, 100),
            Stock::new("BBAS3".to_string(), 32.63, 500),
            Stock::new("BBDC4".to_string(), 24.70, 400),
        ])
        .add_interested_list(vec![
            Interested::new("Marcos Dantas".to_string(), 2000.89),
        ]);

    BrokerDetails::new(&broker)
        .show_details();
}
