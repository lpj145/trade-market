pub struct Display {
    pub buffer: String
}

impl Display {
    pub fn new() -> Self {
        Display {
            buffer: "".to_string()
        }
    }
    pub fn message(&self, message: String) -> &Self {
        println!("{}", message);
        self
    }
}