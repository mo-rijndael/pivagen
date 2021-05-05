pub struct Request {
    pub write_intent: bool,
    pub content: String,
}
impl Request {
    pub fn save(content: String) -> Self {
        Self {
            write_intent: true,
            content,
        }
    }
    pub fn generate(content: String) -> Self {
        Self {
            write_intent: true,
            content,
        }
    }
}
