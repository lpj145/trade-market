use super::interested::Interested;
pub struct Intention<'a> {
    interested: &'a Interested,
    intention_id: u32,
    qty: u32,
    price: f32,
    time: u32,
    cancelled: bool,
    executed: bool,
}

impl<'a> Intention<'a> {
    pub fn new(interested: &'a Interested, intention_id: u32, price: f32, qty: u32) -> Self {
        Intention {
            interested,
            intention_id,
            qty,
            price,
            cancelled: false,
            executed: false,
            time: 0
        }
    }
}