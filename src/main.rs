use yew::prelude::*;
mod npc;
use npc::*;
// mod quest;
// use quest::*;

#[function_component(App)]
fn app() -> Html {
    let npc = NPC::new();
    let npc_name = format!("{} {}", npc.behavior, npc.ancestry);
    // for _ in (0..10).into_iter() {
    //     let quest = Quest::new();
    //     println!("{} in a {}", quest.objective, quest.location);
    // }
    html! {
    <div>
        <h1>{ "Lazy DM's companion" }</h1>
        <h2>{ npc_name }</h2>
    </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
