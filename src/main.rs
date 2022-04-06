mod npc;
use npc::*;
mod quest;
use quest::*;

fn main() {
    let npc = NPC::new();
    println!("{} {}", npc.behavior, npc.ancestry);
    for _ in (0..10).into_iter() {
        let quest = Quest::new();
        println!("{} in a {}", quest.objective, quest.location);
    }
}
