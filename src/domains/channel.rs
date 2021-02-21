use super::enums::IntentionTypes;
pub struct Channel {
    r#type: IntentionTypes,
    pub name: String
}

impl Channel {
    pub fn new(name: String, r#type: IntentionTypes) -> Self {
        Channel {
            r#type,
            name
        }
    }
}