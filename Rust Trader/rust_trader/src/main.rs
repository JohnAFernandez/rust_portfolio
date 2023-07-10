#![allow(dead_code)] // until more of this is written.

mod generation_astra;

fn start_game() -> generation_astra::GameplayState {
    let mut _state: generation_astra::GameplayState = generation_astra::GameplayState::build_gameplay_state(String::from("Test Player Name"), generation_astra::DifficultyLevel::Easy);

//    println!("Showing generated systems.");

//    for star_type in 0..20{
//        println!("Type {} has {} in the gamestate", star_type, state.count_star_type(star_type));
//    }



    _state
}




fn main() {
    // init SDL or some other goofiness here.


    // actually start stuff
    start_game();
    // get input for name and difficulty.
    //build_gameplay_state(player_name, difficulty)
}
