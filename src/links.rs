use rustdoc_types::Id;

use crate::ui::item::ItemOrder;

pub type Links = Vec<Link>;

#[derive(Debug, Clone)]
pub struct Link {
    pub text: String,
    pub id: Id,
    pub order: ItemOrder,
}

impl Link {
    pub fn new(text: String, id: Id, order: ItemOrder) -> Self {
        Self { text, id, order }
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Eq for Link {}

impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Link {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}
