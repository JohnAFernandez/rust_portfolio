use std::collections::{HashMap};

enum WeaponTypes { 
    Lazer,      // High Defense, Low Attack
    Missile,    // Balanced Attack and Defense
    MassDriver  // High Attack, No Additional Defense
}

struct WeaponStats {
    name : String,
    size : f32,             // how much space this would take up on a ship
    power_required : f32,    // how much reactor power it will use when activated
    base_damage : f32,       // how much damage this will cause
    type_ : WeaponTypes
}


enum EquipmentTypes {
    AdvancedSensors
}


enum ShipTypes {
    Fighter,
    Destroyer,
    Frigate,
    Cruiser,
    Battleship,
    Carrier
}

// The immutable basic details for a ship
struct ShipStats {
    class_name : String,
    type_ : ShipTypes,
    max_cargo_volume : f32,
    crew_minimum : i16,
    crew_maximum : i16,
    base_mass: f64,
    sensor_range: f64,
    weapon_slots: i16,
    equipment_slots: i16
}

// The current values needed for a ship (and some copies of Ship Stats)
struct Ship {
    class_name : String,
    name : String,
    type_ : ShipTypes,
    cargo_volume : f32,
    max_cargo_volume : f32,

    current_crew : i16,
    crew_minimum : i16,
    crew_maximum : i16,
    
    passengers: i16, // max determined by cargo maximum
    base_mass: f64,
    current_mass: f64,

    hull_integrity: f64,
    armor_level: i16,
    max_engine_thrust: f64,
    engine_health: f32,
    
    weapon_slots: i16,
    equipment_slots : i16,
    weapons : Vec<String>,
    equipment : Vec<String>,

    ai : AIValues
}

enum Factions{
    Traders,
    Police,
    Pirates,
    Aliens
}

struct AIValues { // In this simple version, I'm not sure this one is needed.
    faction: Factions
}

struct Player {
    name : String,
    credits : i64,
    reputation : i16,
    morality : i16,
    current_ship : String
}

fn build_player(name: String, credits: i64) -> Player {
    Player {
        name,
        credits,
        reputation : 0,
        morality : 0,
        current_ship : String::from("Test")
    }
}

// Are these going to do anything yet?  I'm not sure.
enum StarTypes {
    O,
    B,
    A,
    F,
    G,
    K,
    L,
    T,
    Y,
    D,
    N,
    BL
}


// To be expanded in the full game.
enum IndustryTypes {
    Water,
    Food,
    Mining,
    CapitalPrinting, // 3D printing of captial machinary used in other manufacturing
    MetalMan,
    Construction,
    Textile,
    ChildCare,
    ChemicalMan,
    Appliance,
    Farmasutical,
    Wellness,
    Entertainment,
    Tourism,
    SpaceMan,
    NanoMan
}

struct IndustryStats{
    type_: IndustryTypes,
    efficiency : f32,
    requires : i8
}


struct Industry {
    name : String,
    capacity : f64,
    employees : i128,
    efficiency : f32,
    requires : i8,
    type_ : IndustryTypes
}

enum WorldSupports {
    Oxygenation = 1 << 0,
    WaterCycle = 1 << 1,
    RawMinerals = 1 << 2,
    Farming = 1 << 3,
    AnimalBiology = 1 << 4,
    EarthGravity = 1 << 5,
    TolerableDisasters = 1 << 6,
    EarthLike = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4 | 1 << 5 | 1 << 6,
}

struct World {
    name : String,
    industries : Vec<Industry>,
    population : i128,
    supports : i8 // What is earthlike on this planet from the WorldSupports Enum
}

struct ResourceStats{
    name : String,
    type_ : IndustryTypes,
    efficiency : i8,
    demand : f32,   // not sure how to implement that yet.
    illegal : bool
}

struct Resource{
    name : String,
    amount : f64,   // in Volume
    illegal : bool
}

struct TradeHub{
    name: String,
    goods : Resource,
    weapons : Vec<String>,
    equipment : Vec<String>
    // Missions might be a good thing to try here.
}

struct System {
    location : Tuple<f32, f32>,
    gdp : i64,
    star_type : StarTypes,
    worlds : Vec<World>,
    space_materials : f64,
    police_presence : f32,
    pirate_presence : f32
}

struct GameplayState{
    ship_stats: HashMap<String, ShipStats>,
    ship_: Vec<Ship>, // Only a few ships will be here at any point, because we have RNG for generating random encounters instead.

    weapon_stats: HashMap<String, WeaponStats>,
    
    systems: Vec<System>,

    player: Player

}

enum DifficultyLevel{
    Easy,
    Medium,
    Hard,
    Impossible
}

fn build_gameplaystate(player_name: String, difficulty: DifficultyLevel) -> GameplayState {

    let mut credits:i64 = 0;

    match difficulty{
        DifficultyLevel::Easy => credits = 10000,
        DifficultyLevel::Medium => credits = 5000,
        DifficultyLevel::Hard => credits = 3000,
        DifficultyLevel::Impossible => credits = 1500,
        _=> println!("Unhandled type in build_gameplaystate")
    }

    GameplayState { ship_stats: HashMap::new(), ship_: Vec::new(), weapon_stats: HashMap::new(), systems: Vec::new(), player: build_player(player_name, credits) }


}

fn start_game() {

    // set up everything the game needs here
    let mut ship_dict: HashMap<String, ShipStats> = HashMap::new();
    let mut systems_vec: Vec<System> = Vec::new();
}

fn main() {

    // get input for name and difficulty.
    //build_gameplaystate(player_name, difficulty)
}
