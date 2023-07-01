#![allow(dead_code)] // until more of this is written.

mod dark_void;

fn start_game() -> dark_void::GameplayState {
    let mut _state: dark_void::GameplayState = dark_void::GameplayState::build_gameplay_state(String::from("Test Player Name"), dark_void::DifficultyLevel::Easy);

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
