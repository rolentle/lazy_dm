#[derive(Debug)]
pub struct NPC {
    pub behavior: String,
    pub ancestry: String,
}

impl NPC {
    pub fn new() -> NPC {
        let behavior = "Enthusiastic".to_string();
        let ancestry = "Human".to_string();
        NPC { 
            behavior, 
            ancestry 
        }
    }
}
