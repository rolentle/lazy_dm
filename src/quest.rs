use rand::thread_rng;
use rand::seq::SliceRandom;
use handlebars::Handlebars;
use serde_json::json;

const ITEMS: [&str; 20] = ["Coin", "Figurine", "Gemstone", "Amulet", "Earring", "Bell", "Bone", "Bowl", "Candle", "Ring", "Circlet", "Bracelet", "Dagger", "Goblet", "Key", "Lamp", "Brooch", "Skull", "Mask", "Necklace"];

pub enum ObjectiveConcern {
    Item,
    // Monument,
    // NPC
}


pub const OBJECTIVES: [(&str, Option<ObjectiveConcern>); 20] = [
    ("Find a(n) {{item}}", Some(ObjectiveConcern::Item)), 
    ("Open a gate", None),
    ("Kill a villain", None),
    ("Activate a monument", None),
    ("Rescue a(n) NPC", None),
    ("Disable an artifact", None),
    ("Uncover a secret", None),
    ("Recover a(n) {{item}}", Some(ObjectiveConcern::Item)),
    ("Clear out monsters", None),
    ("Convince an NPC", None),
    ("Protect a monument", None),
    ("Awaken a monster", None),
    ("Protect an NPC", None),
    ("Put a monster to sleep", None),
    ("Steal a(n) {{item}}", Some(ObjectiveConcern::Item)),
    ("Bury a secret", None),
    ("Return a(n) {{item}}", Some(ObjectiveConcern::Item)),
    ("Discover a monument", None),
    ("Close a gate", None),
    ("Dig up an artifact", None)];

const LOCATIONS: [&str; 20] = ["Tower", "Crypts", "Keep", "Cairn", "Giant", "Caves", "Sewers", "Temple", "Mines", "Mansion", "Academy", "Dungeon", "Barrow", "Vault", "Tomb", "Warren", "Ship", "Sanctum", "Cove", "Castle"];

// const MONUMENTS: [&str; 20] = ["Sarcophagus", "Obelisk", "Orb", "Bone pile", "Skull", "Megalith", "Pillars", "Throne", "Statues", "Well", "Orrery", "Effigy", "Arcane circle", "Spire", "Altar", "Pit", "Fountain", "Archway", "Cage", "Brazier"];


pub struct Quest {
    pub objective: String,
    pub location: String,
}

fn get_objective() -> String {
    let reg = Handlebars::new();
    let mut rng = thread_rng();
    let(base_objective, concern) = OBJECTIVES.choose(&mut rng).unwrap();
    match concern {
        Some(concern) => match concern {
            ObjectiveConcern::Item => reg.render_template(base_objective, &json!({"item": get_item().to_string()})).unwrap(),
        }
        None => base_objective.to_string()
    }
}

fn get_item() -> String {
    let mut rng = thread_rng();
    ITEMS.choose(&mut rng).unwrap().to_string()
}

impl Quest {
    pub fn new() -> Quest {
        let mut rng = thread_rng();
        let objective = get_objective();
        let location = LOCATIONS.choose(&mut rng).unwrap().to_string();
        Quest { 
            objective, 
            location 
        }
    }

}
