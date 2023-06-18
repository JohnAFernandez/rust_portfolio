use std::collections::{HashMap};



enum WeaponTypes { 
    Lazer,      // High Defense, Low Attack
    Missile,    // Balanced Attack and Defense
    MassDriver  // High Attack, No Additional Defense
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


enum Factions{
    Traders,
    Police,
    Pirates,
    Aliens
}


enum WorldSupports {
    Oxygenation = 1 << 0,
    WaterCycle = 1 << 1,        
    RawMinerals = 1 << 2,       // False means just Carbon, Gas Giant, or Iceball
    Farming = 1 << 3,           // False means no carbon
    AnimalBiology = 1 << 4,     
    EarthGravity = 1 << 5,
    TolerableDisasters = 1 << 6,
    ToxicAtmosphere = 1 << 7,
    ToxicOceans = 1 << 8,
    NoAtmosphere = 1 << 9,
    TidallyLocked = 1 << 10,    // Specifically with parent star
    InsideHabitalZone = 1 << 11,
    HighPressureAtmosphere = 1 << 12,   // Not a good thing.  Think Venus
    Hydrogen = 1 << 13,
    Rings = 1 << 14,
    NaturalSatellites = 1 << 15,
}


enum WorldTypes {
    EarthLike = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4 | 1 << 5 | 1 << 6 | 1 << 11,
    BiologicalGasGiant = 1 << 0 | 1 << 1 | 1 << 4 | 1 << 11 | 1 << 13,
    MercuryLike = 1 << 2 | 1 << 9 | 1 << 10,
    VenusLike = 1 << 2 | 1 << 5 | 1 << 7 | 1 << 11 | 1 << 12,
    MarsLike = 1 << 2 | 1 << 5
}


enum DifficultyLevel{
    Easy,
    Medium,
    Hard,
    Impossible
}


// Main Gameplay objects and constructors.
struct WeaponStats {
    name : String,
    size : f32,             // how much space this would take up on a ship
    power_required : f32,    // how much reactor power it will use when activated
    base_damage : f32,       // how much damage this will cause
    type_ : WeaponTypes
}

fn build_WeaponStats(name : String, size : f32, power_required : f32, base_damage : f32, type_ : WeaponTypes) -> WeaponStats {
    WeaponStats { name, size, power_required, base_damage, type_ }
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

fn build_ShipStats(class_name : String, type_ : ShipTypes, max_cargo_volume : f32, crew_minimum : i16, crew_maximum : i16, base_mass: f64, sensor_range: f64, weapon_slots: i16, equipment_slots: i16) -> ShipStats {
    ShipStats { class_name, type_, max_cargo_volume, crew_minimum, crew_maximum, base_mass, sensor_range, weapon_slots, equipment_slots }
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

fn build_Ship(class_name : String, name : String, type_ : ShipTypes, cargo_volume : f32, max_cargo_volume : f32, current_crew : i16, crew_minimum : i16, crew_maximum : i16,
    passengers: i16, base_mass: f64, current_mass: f64, hull_integrity: f64, armor_level: i16, max_engine_thrust: f64, engine_health: f32, weapon_slots: i16, equipment_slots : i16,
    weapons : Vec<String>, equipment : Vec<String>, ai : AIValues) -> Ship {

    Ship{class_name, name, type_, cargo_volume, max_cargo_volume, current_crew, crew_minimum, crew_maximum, passengers, base_mass, current_mass, hull_integrity, armor_level,
    max_engine_thrust, engine_health, weapon_slots, equipment_slots, weapons, equipment, ai}
}


struct AIValues { // In this simple version, I'm not sure this one is needed.
    faction: Factions
}

fn build_AIValues(faction: Factions) -> AIValues {
    AIValues {
        faction
    }
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


struct IndustryStats{
    type_: IndustryTypes,
    efficiency : f32,
    requires : i16  // from worldsupports
}

fn build_IndustryStats(type_: IndustryTypes, efficiency : f32, requires : i16) -> IndustryStats {
    IndustryStats {
        type_, efficiency, requires
    }
}


struct Industry {
    name : String,
    capacity : f64,
    employees : i128,
    efficiency : f32,
    requires : i16,
    type_ : IndustryTypes
}

fn build_Industry(name : String, capacity : f64, employees : i128, efficiency : f32, requires : i16, type_ : IndustryTypes) -> Industry {
    Industry {name, capacity, employees, efficiency, requires, type_}
}


struct World {
    name : String,
    industries : Vec<Industry>,
    population : i128,
    supports : i16 // What is earthlike on this planet from the WorldSupports Enum
    
}

fn build_World (name : String, industries : Vec<Industry>, population : i128, supports : i16) -> World {
    World{ name, industries, population, supports}
}


struct ResourceStats{
    name : String,
    type_ : IndustryTypes,
    efficiency : i8,
    demand : f32,   // not sure how to implement that yet.
    illegal : bool
}

fn build_ResourceStats( name : String, type_ : IndustryTypes, efficiency : i8, demand : f32, illegal : bool) -> ResourceStats {
    ResourceStats { name, type_, efficiency, demand, illegal}
}


struct Resource{
    name : String,
    amount : f64,   // in Volume
    illegal : bool
}

fn build_Reource(name : String, amount : f64, illegal : bool) -> Resource {
    Resource{ name, amount, illegal}
}


struct TradeHub{
    name: String,
    goods : Resource,
    weapons : Vec<String>,
    equipment : Vec<String>,
    location : Location
    // Missions might be a good thing to try here.
}

fn build_TradeHub(name: String, goods : Resource, weapons : Vec<String>, equipment : Vec<String>, location : Location) -> TradeHub {
    TradeHub { name, goods, weapons, equipment, location}
}


struct Location {
    system_name : String,
    orbit_level : i16
}

fn build_Location(system_name : String, orbit_level : i16) -> Location {
    Location {system_name, orbit_level}
}

struct System {
    location : Location,
    gdp : i64,
    star_type : StarTypes,
    worlds : Vec<World>,
    space_materials : f64,
    police_presence : f32,
    pirate_presence : f32
}


// Every time we need the gameplay state to make a decision in a new way, this struct needs to change to encorporate that type of task, using a new task of its own.
// Don't currently have any, but that is going to need to change to have any chance of finishing this monstrosity.
struct TaskStack{
    fish : i16
}

fn build_TaskStack(fish : i16) -> TaskStack{
    TaskStack{fish}
}

// Every time we need the gameplay stack to actually change, this struct needs to have a new result added to it. This stack being empty is the trigger for copying relevant object locations to the renderer.
// Lol, this is going to take forever.
struct ResultStack{
    fish : i16
}

// gameplay state and the gameplay tasks really need their own file.
struct GameplayState{
    ship_stats: HashMap<String, ShipStats>,
    ship_: Vec<Ship>, // Only a few ships will be here at any point, because we have RNG for generating random encounters instead.

    weapon_stats: HashMap<String, WeaponStats>,
    
    systems: Vec<System>,

    player: Player,

    sim_time : i128,
    tasks: HashMap<i128, TaskStack>,
    results: ResultStack,
    multiplayer_stack: HashMap<i128, TaskStack>
}


fn build_gameplaystate(player_name: String, difficulty: DifficultyLevel) -> GameplayState {

    let mut credits:i64 = 0;

    match difficulty{
        DifficultyLevel::Easy => credits = 10000,
        DifficultyLevel::Medium => credits = 5000,
        DifficultyLevel::Hard => credits = 3000,
        DifficultyLevel::Impossible => credits = 1500,
        //_=> println!("Unhandled type in build_gameplaystate")  // This triggers a warning....
    }

    GameplayState { ship_stats: HashMap::new(), ship_: Vec::new(), weapon_stats: HashMap::new(), systems: Vec::new(), player: build_player(player_name, credits) }


}

fn start_game() {
    let state = build_gameplaystate(String::from("Test Player Name"), DifficultyLevel::Easy);


}

fn main() {
    start_game()
    // get input for name and difficulty.
    //build_gameplaystate(player_name, difficulty)
}
