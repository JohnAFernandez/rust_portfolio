use std::collections::HashMap;
use std::ops;
use std::num;
use rand::Rng;
use sqlite;



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


enum WC { // short for World Characteristics
    // Good Stuff
    Oxygenation = 1 << 0,       // Creates Ozone, so no need to add Ozone.
    WaterCycle = 1 << 1,        
    RawMinerals = 1 << 2,       // False means just Carbon, Gas Giant, or Iceball
    NaturalSoil = 1 << 3,           // False means no carbon, but it essentially means that to do farming, soil does not need to be imported.
    NaturalAnimalBiology = 1 << 4,     
    EarthGravity = 1 << 5,
    TolerableDisasters = 1 << 6,
    Hydrogen = 1 << 7,
    InsideHabitalZone = 1 << 8,
    MagneticField = 1 << 9,    // this is a prerequisite for life.


    // Bad stuff
    ToxicAtmosphere = 1 << 11,
    ToxicOceans = 1 << 12,
    NoAtmosphere = 1 << 13,
    MinimalAtmosphere = 1 << 14,
    TidallyLocked = 1 << 15,    // Specifically with parent star, no one cares if it's tidally locked with a satellite or a non-star parent.
    HighPressureAtmosphere = 1 << 16,   // Not a good thing.  Think Venus
    ExtremeHeat = 1 << 17,
    ExtremeCold = 1 << 18,
    Spheroid = 1 << 19,
    NuclearWinter = 1 << 20,

    // Nutral stuff
    Rings = 1 << 22,
    Oceans = 1 << 23,
    TectonicallyActive = 1 << 24,
    NaturalSatellites = 1 << 25,


}

impl ops::BitOr<WC> for WC {
    type Output = isize;

    fn bitor(self, rhs: WC) -> Self::Output {
        return self as isize | rhs as isize
    }
}

impl ops::BitOr<isize> for WC {
    type Output = isize;

    fn bitor(self, rhs: isize) -> Self::Output {
        return self as isize | rhs
    }
}

impl ops::BitOr<WC> for isize {
    type Output = isize;

    fn bitor(self, rhs: WC) -> Self::Output {
        return self | rhs as isize 
    }
}


// still not sure I'm going to use this.
enum WorldTypes {
    EarthLike = WC::Oxygenation as isize | WC::WaterCycle as isize | 1 << 2 | 1 << 3 | 1 << 4 | 1 << 5 | 1 << 6 | 1 << 11 | 1 << 16 | 1 << 17,
    BiologicalGasGiant = 1 << 0 | 1 << 1 | 1 << 4 | 1 << 11 | 1 << 13 | 1 << 17,
    MercuryLike = 1 << 2 | 1 << 9 | 1 << 10,
    VenusLike = 1 << 2 | 1 << 5 | 1 << 7 | 1 << 11 | 1 << 12,
    MarsLike = WC::RawMinerals as isize | 1 << 5
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
    requires : i32  // from WorldCharacterists
}

fn build_IndustryStats(type_: IndustryTypes, efficiency : f32, requires : i32) -> IndustryStats {
    IndustryStats {
        type_, efficiency, requires
    }
}


struct Industry {
    name : String,
    capacity : f64,
    employees : i128,
    efficiency : f32,
    requires : i32,
    type_ : IndustryTypes
}

fn build_Industry(name : String, capacity : f64, employees : i128, efficiency : f32, requires : i32, type_ : IndustryTypes) -> Industry {
    Industry {name, capacity, employees, efficiency, requires, type_}
}


struct World {
    name : String,
    mass : f64,
    industries : Vec<Industry>,
    population : i128,
    supports : i32 // What is earthlike on this planet from the WorldCharacterists Enum
    
}

fn build_World (name : String, mass : f64, industries : Vec<Industry>, population : i128, supports : i32) -> World {
    World{ name, mass, industries, population, supports}
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

fn build_Resource(name : String, amount : f64, illegal : bool) -> Resource {
    Resource{ name, amount, illegal}
}


struct TradeHub{
    name: String,
    goods : Resource,
    weapons : Vec<String>,
    equipment : Vec<String>,
    orbit : Orbit
    // Missions might be a good thing to try here.
}

fn build_TradeHub(name: String, goods : Resource, weapons : Vec<String>, equipment : Vec<String>, orbit : Orbit) -> TradeHub {
    TradeHub { name, goods, weapons, equipment, orbit}
}


struct Orbit {
    system_name : String,
    orbit_level : i16
}

impl Orbit {
    fn build_Orbit(system_name : String, orbit_level : i16) -> Orbit {
        Orbit {system_name, orbit_level}
    }
   
}

struct StarmapLocation {
    x : f32,
    y : f32
}

impl StarmapLocation{
    fn build_StarmapLocation(x : f32, y : f32) -> StarmapLocation{
        StarmapLocation {x, y}
    }

    const MINIMUM_DISTANCE : f32 = 5.0;

    fn find_system_distance(lhs : &StarmapLocation, rhs : &StarmapLocation) -> f32 {
        ((lhs.x - rhs.x).powf(2.0) + (lhs.y - rhs.y).powf(2.0)).sqrt()
    }

    fn find_distance_to_point(x : f32, y : f32, rhs : &StarmapLocation) -> f32 {
        ((x - rhs.x).powf(2.0) + (y - rhs.y).powf(2.0)).sqrt()        
    }

    fn build_random_starmap_location(systems : &Vec<System>) -> StarmapLocation{
        let mut overlap : bool = true;

        let mut x : f32 = 0.0;
        let mut y : f32 = 0.0;

        let mut rng = rand::thread_rng();

        while overlap == true{
            overlap = false;
            
            x = rng.gen_range(0.0 .. 1000.0);
            y = rng.gen_range(0.0 .. 1000.0);


            for star in systems {
                overlap = StarmapLocation::find_distance_to_point(x, y, &star.location) < StarmapLocation::MINIMUM_DISTANCE;
                
                if overlap {
                    break
                }
            }
    
        }

        StarmapLocation::build_StarmapLocation(x, y)
    }    
}

struct System {
    location : StarmapLocation,
    gdp : i64,
    star_type : StarTypes,
    worlds : Vec<World>,
    space_materials : f64,
    police_presence : f32,
    pirate_presence : f32
}

impl System {
    fn build_System(location : StarmapLocation, gdp : i64, star_type : StarTypes, worlds : Vec<World>, space_materials : f64, police_presence : f32, pirate_presence : f32) -> System{
        System{location, gdp, star_type, worlds, space_materials, police_presence, pirate_presence}
    }    

    fn build_random_system(gs : &GameplayState) -> System {
        let location : StarmapLocation = StarmapLocation::build_random_starmap_location(&gs.systems);

        System::build_System(location, 10000000, StarTypes::O, Vec::new(), 0.0, 95.0, 5.0)
    }
}

// Every time we need the gameplay state to make a decision in a new way, this struct needs to change to encorporate that type of task, using a new task of its own.
// Don't currently have any, but that is going to need to change to have any chance of finishing this monstrosity.
struct TaskStack{
    fish : i16
}

fn build_TaskStack(fish : i16) -> TaskStack{
    TaskStack{fish}
}


// Every time we need the gameplay stack to actually change, this struct needs to have a new result added to it. This stack being empty is the trigger for copying relevant object orbits to the renderer.
// Lol, this is going to take forever.
struct ResultStack{
    fish : i16
}

fn build_ResultStack(fish : i16) -> ResultStack{
    ResultStack {fish}
}

// gameplay state and the gameplay tasks really need their own file.
struct GameplayState{
    ship_stats: HashMap<String, ShipStats>,
    ship_: Vec<Ship>, // Only a few ships will be here at any point, because we have RNG for generating random encounters instead.

    weapon_stats: HashMap<String, WeaponStats>,
    
    systems: Vec<System>,

    player: Player,

    sim_time : u128,
    tasks: HashMap<u128, TaskStack>,
    results: ResultStack,
    multiplayer_stack: HashMap<u128, TaskStack>
}

impl GameplayState {
    fn add_random_system(&mut self) {
        self.systems.push(System::build_random_system(self))
    }
}

fn build_GameplayState(player_name: String, difficulty: DifficultyLevel) -> GameplayState {

    let mut credits:i64 = 0;

    match difficulty{
        DifficultyLevel::Easy => credits = 10000,
        DifficultyLevel::Medium => credits = 5000,
        DifficultyLevel::Hard => credits = 3000,
        DifficultyLevel::Impossible => credits = 1500,
        //_=> println!("Unhandled type in build_GameplayState")  // This triggers a warning....
    }

    let sim_time  = 0;

    GameplayState { ship_stats: HashMap::new(), ship_: Vec::new(), weapon_stats: HashMap::new(), systems: Vec::new(), player: build_player(player_name, credits), sim_time, tasks: HashMap::new(), results : build_ResultStack(0), multiplayer_stack : HashMap::new() }


}

fn start_game() {
    let mut state: GameplayState = build_GameplayState(String::from("Test Player Name"), DifficultyLevel::Easy);

    let mut debug_counter = 0;

    while state.systems.len() < 100{
        state.add_random_system();
        debug_counter +=1;

        if state.systems.len() != debug_counter {
            println!("We have a problem. System size {} debug counter {}", state.systems.len(), debug_counter);
        }
    }

    println!("Showing generated systems.");

    for tom in state.systems{
        println!("{} {}", tom.location.x, tom.location.y);
    }
}

fn main() {
    // init SDL or some other goofiness here.


    // actually start stuff
    start_game()
    // get input for name and difficulty.
    //build_GameplayState(player_name, difficulty)
}
