use super::enums::INTENTION_TYPES;
pub struct Channel {
    r#type: INTENTION_TYPES,
    pub name: String
}

impl Channel {
    pub fn new(name: String, r#type: INTENTION_TYPES) -> Self {
        Channel {
            r#type,
            name
        }
    }
}