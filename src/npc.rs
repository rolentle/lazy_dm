use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct NPC {
    pub behavior: String,
    pub ancestry: String,
}

const BEHAVIORS: [&str; 20] = ["Enthusiastic", "Flighty", "Shifty", "Optimistic", "Paranoid", "Well spoken", "Superior", "Haughty", "Pessimistic", "Suspicious", "Worried", "Greedy", "Brave", "Stern", "Sly", "Wise", "Reserved", "Cheery", "Opportunistic", "Soft spoken"];

const ANCESTORIES: [&str; 20] = ["Human", "Elf", "Dwarf", "Halfling", "Orc", "Drow", "Tiefling", "Dragonborn", "Fey", "Goblin", "Construct", "Celestial", "Ghost", "Wizard's familiar", "Talking animal", "Avian", "Lizardfolk", "Catfolk", "Lycanthrope", "Artifact"];

impl NPC {
    pub fn new() -> NPC {
        let mut rng = thread_rng();
        let behavior = BEHAVIORS.choose(&mut rng).unwrap().to_string();
        let ancestry = ANCESTORIES.choose(&mut rng).unwrap().to_string();
        NPC { 
            behavior, 
            ancestry 
        }
    }
}
