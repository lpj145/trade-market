mod domains;

use domains::broker::Broker;

fn main() {
    print!("Welcome to market share.\n");
    let mut broker = Broker::new();
    broker.add_stock_list(vec![
        broker.factory_stock("MGLU3".to_string(), 24.94, 500),
        broker.factory_stock("PCAR3".to_string(), 64.75, 200),
        broker.factory_stock("NTCO3".to_string(), 52.55, 100),
        broker.factory_stock("BIDI11".to_string(), 159.14, 3000),
        broker.factory_stock("BBAS3".to_string(), 32.63, 500),
        broker.factory_stock("BBDC4".to_string(), 24.70, 400),
    ]);
    broker.print_broker_details();
}
