mod npc;
use npc::*;

fn main() {
    let npc = NPC::new();
    println!("{} {}", npc.behavior, npc.ancestry);
}
