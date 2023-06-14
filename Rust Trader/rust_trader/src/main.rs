struct ShipType {
    cargo_volume : f32,
    max_cargo_volume : f32,

    current_crew : i16,
    crew_minimum : i16,
    crew_maximum : i16,
    
    passengers: i16, // max determined by cargo maximum
    base_mass: f64,
    current_mass: f64,


    name : str
    
}


struct Player {
    credits : i64,
    reputation : i16,
    morality : i16,
    current_ship : ShipType
}


fn start_game() {
    let mut fox = Shuo


    player : Player(1000, 0, 0, ) // lol, I need to know how to instantiate things.
}

fn main() {

    // where's my game state?

    // WHERE IS MAI GAME STATE!!

    start_game()
}
