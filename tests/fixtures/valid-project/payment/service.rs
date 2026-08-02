pub struct Order {
    pub id: u64,
}

pub fn total(order: &Order) -> u64 {
    order.id
}
