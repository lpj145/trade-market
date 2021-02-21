mod domains;

use domains::{
    broker::Broker,
    stock::Stock,
    channel::Channel,
    interested::Interested,
    enums::IntentionTypes,
    broker_details::BrokerDetails,
    intention::Intention
};

fn main() {
    let mut interest_list: Vec<Interested> = vec![
        Interested::new("Marcos Dantas".to_string(), 2000.90),
        Interested::new("Ane Dantas".to_string(), 20000.90)
    ];
    let marcos = Interested::new("Marcos Dantas".to_string(), 2000.90);
    let ane = Interested::new("Ane Dantas".to_string(), 20000.90);
    let mut broker = Broker::new();

    broker
        .add_channel(Channel::new("Comprar Ações".to_string(), IntentionTypes::BuyStock))
        .add_channel(Channel::new("Vender Ações".to_string(), IntentionTypes::SellStock))
        .add_stock_list(vec![
            Stock::new("MGLU3".to_string(), 24.94, 500),
            Stock::new("PCAR3".to_string(), 64.75, 200),
            Stock::new("BIDI11".to_string(), 159.14, 3000),
            Stock::new("NTCO3".to_string(), 52.55, 100),
            Stock::new("BBAS3".to_string(), 32.63, 500),
            Stock::new("BBDC4".to_string(), 24.70, 400),
        ])
        .add_interested_list(vec![&marcos, &ane]);

    for interest in &interest_list {
        let intetion_id = broker.get_next_intetion_id();
        broker.add_intention(
            Intention::new(&interest, intetion_id, 20.0, 20)
        );
    }

    BrokerDetails::new(&broker)
        .show_details();
}
