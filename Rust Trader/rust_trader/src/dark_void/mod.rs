#![allow(dead_code)] // until more of this is written.

use std::collections::HashMap;
//use std::isize;
//use std::ops;
//use std::num;
use rand::Rng;
//use sqlite;

mod star_calcs;

enum WeaponTypes { 
    Lazer,      // High Defense, Low Attack
    Missile,    // Balanced Attack and Defense
    MassDriver  // High Attack, No Additional Defense
}

// Main Gameplay objects and constructors.
struct WeaponStats {
    name : String,
    size : f32,             // how much space this would take up on a ship
    power_required : f32,    // how much reactor power it will use when activated
    base_damage : f32,       // how much damage this will cause
    type_ : WeaponTypes
}

fn build_weapon_stats(name : String, size : f32, power_required : f32, base_damage : f32, type_ : WeaponTypes) -> WeaponStats {
    WeaponStats { name, size, power_required, base_damage, type_ }
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


enum Factions{
    Traders,
    Police,
    Pirates,
    Aliens
}


pub enum DifficultyLevel{
    Easy,
    Medium,
    Hard,
    Impossible
}


// The immutable basic details for a ship
struct ShipStats {
    class_name : String,
    type_ : i8,
    max_cargo_volume : f32,
    crew_minimum : i16,
    crew_maximum : i16,
    base_mass: f64,
    sensor_range: f64,
    weapon_slots: i16,
    equipment_slots: i16
}

impl ShipStats{

    const TYPES : [&str ; 6] = ["Fighter", "Destroyer", "Frigate", "Cruiser", "Battleship", "Carrier"];

    fn build_ship_stats(class_name : String, type_ : i8, max_cargo_volume : f32, crew_minimum : i16, crew_maximum : i16, base_mass: f64, sensor_range: f64, weapon_slots: i16, equipment_slots: i16) -> ShipStats {
        ShipStats { class_name, type_, max_cargo_volume, crew_minimum, crew_maximum, base_mass, sensor_range, weapon_slots, equipment_slots }
    }    
}


// The current values needed for a ship (and some copies of Ship Stats)
struct Ship {
    class_name : String,
    name : String,
    type_ : i8,
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
}

impl Ship {
    
    fn build_ship(class_name : String, name : String, type_ : i8, cargo_volume : f32, max_cargo_volume : f32, current_crew : i16, crew_minimum : i16, crew_maximum : i16,
        passengers: i16, base_mass: f64, current_mass: f64, hull_integrity: f64, armor_level: i16, max_engine_thrust: f64, engine_health: f32, weapon_slots: i16, equipment_slots : i16,
        weapons : Vec<String>, equipment : Vec<String>) -> Ship {
    
        Ship{class_name, name, type_, cargo_volume, max_cargo_volume, current_crew, crew_minimum, crew_maximum, passengers, base_mass, current_mass, hull_integrity, armor_level,
        max_engine_thrust, engine_health, weapon_slots, equipment_slots, weapons, equipment}
    }

}


struct AIValues { // In this simple version, I'm not sure this one is needed.
    faction: Factions
}

fn build_ai_values(faction: Factions) -> AIValues {
    AIValues {
        faction
    }
}


struct Player {
    name : String,
    credits : i64,
    reputation : i16,
    morality : i16,
    ship : Ship
}

fn build_player(name: String, credits: i64) -> Player {
    Player {
        name,
        credits,
        reputation : 0,
        morality : 0,
        ship : Ship::build_ship(String::from("Test Ship Class"), String::from("Test player ship name"), 0, 0.0, 100.0, 1, 1, 2, 0, 30000.0, 30000.0, 1.0, 1, 1000.0, 1.00, 2, 0, Vec::new(), Vec::new())
    }
}


struct IndustryStats{
    type_: IndustryTypes,
    efficiency : f32, // how many man hours per kilogram ...
    requires : i32  // from WorldCharacterists
}

fn build_industry_stats(type_: IndustryTypes, efficiency : f32, requires : i32) -> IndustryStats {
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

fn build_industry(name : String, capacity : f64, employees : i128, efficiency : f32, requires : i32, type_ : IndustryTypes) -> Industry {
    Industry {name, capacity, employees, efficiency, requires, type_}
}


struct ResourceStats{
    name : String,
    type_ : IndustryTypes,
    efficiency : i8,
    demand : f32,   // not sure how to implement that yet.
    illegal : bool
}

fn build_resource_stats( name : String, type_ : IndustryTypes, efficiency : i8, demand : f32, illegal : bool) -> ResourceStats {
    ResourceStats { name, type_, efficiency, demand, illegal}
}


struct Resource{
    name : String,
    amount : f64,   // in Volume
    illegal : bool
}

fn build_resource(name : String, amount : f64, illegal : bool) -> Resource {
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

fn build_trade_hub(name: String, goods : Resource, weapons : Vec<String>, equipment : Vec<String>, orbit : Orbit) -> TradeHub {
    TradeHub { name, goods, weapons, equipment, orbit}
}


struct Orbit {
    system_name : String,
    orbit_level : i16,
    distance : f32
}

impl Orbit {
    fn build_orbit(system_name : String, orbit_level : i16, distance : f32) -> Orbit {
        Orbit {system_name, orbit_level, distance}
    }
   
}

struct World {
    name : String,
    mass : f64,
    industries : Vec<Industry>,
    population : i128,
    supports : i64 // What is earthlike on this planet from the WorldCharacterists Enum
    
}

impl World{
    // Good Stuff
    const OXYGENATION : i64 = 1 << 0;       // Creates Ozone, so no need to add Ozone.
    const WATER_CYCLE : i64 = 1 << 1;        
    const RAW_MATERIALS : i64 = 1 << 2;       // False means, Gas Giant, or Iceball
    const NATURAL_SOIL : i64 = 1 << 3;           // False means no carbon, but it essentially means that to do farming, soil does not need to be imported.
    const NATURAL_ANIMAL_BIOLOGY : i64 = 1 << 4;     
    const EARTH_GRAVITY : i64 = 1 << 5;
    const TOLDERABLE_DISASTERS : i64 = 1 << 6;
    const HYDROGEN : i64 = 1 << 7;
    const INSIDE_HABITABLE_ZONE : i64 = 1 << 8;
    const MAGNETIC_FIELD : i64 = 1 << 9;    // this is a prerequisite for life.

    // Bad stuff
    const TOXIC_ATMOSPHERE : i64 = 1 << 11;
    const TOXIC_OCEANS : i64 = 1 << 12;
    const NO_ATMOSPHERE : i64 = 1 << 13;
    const MINIMAL_ATMOSPHERE : i64 = 1 << 14;
    const TIDALLY_LOCKED : i64 = 1 << 15;    // Specifically with parent star, no one cares if it's tidally locked with a satellite or a non-star parent.
    const HIGH_PRESSURE_ATMOSPHERE : i64 = 1 << 16;   // Not a good thing.  Think Venus
    const EXTREME_HEAT : i64 = 1 << 17;
    const EXTREME_COLD : i64 = 1 << 18;
    const SPHEROID : i64 = 1 << 19;
    const NUCLEAR_WINTER : i64 = 1 << 20;
    const ACIDIC : i64 = 1 << 21;
    const ALKALINE : i64 = 1 << 22;
    const HIGH_GRAVITY : i64 = 1 << 23;

    // Nutral stuff
    const RINGS : i64 = 1 << 25;
    const OCEANS : i64 = 1 << 26;
    const TECTONICALLY_ACTIVE : i64 = 1 << 27;
    const NATURAL_SATELLITES : i64 = 1 << 28;

    // Aggregates
    const MERCURY_LIKE : i64 = World::RAW_MATERIALS | World::EXTREME_COLD | World::EXTREME_HEAT | World::NO_ATMOSPHERE | World::TIDALLY_LOCKED;    
    const VENUS_LIKE : i64 = World::RAW_MATERIALS | World::ACIDIC | World::HIGH_PRESSURE_ATMOSPHERE | World::EARTH_GRAVITY | World::EXTREME_HEAT | World::INSIDE_HABITABLE_ZONE;
    const EARTH_LIKE : i64 = World::OXYGENATION | World::WATER_CYCLE | World::RAW_MATERIALS | World::NATURAL_SOIL | World::NATURAL_ANIMAL_BIOLOGY | World::EARTH_GRAVITY | World::TOLDERABLE_DISASTERS | World::HYDROGEN | World::INSIDE_HABITABLE_ZONE | World::MAGNETIC_FIELD | World::OCEANS | World::TECTONICALLY_ACTIVE | World::NATURAL_SATELLITES;
    const CURRENT_EARTH : i64 = World::EARTH_LIKE | World::NUCLEAR_WINTER | World::TOXIC_ATMOSPHERE;
    const MARS_LIKE : i64 = World::INSIDE_HABITABLE_ZONE | World::EXTREME_COLD | World::NATURAL_SATELLITES | World::MINIMAL_ATMOSPHERE | World::RAW_MATERIALS | World::MAGNETIC_FIELD;
    const JUPITER_LIKE : i64 = World::HIGH_GRAVITY | World::MAGNETIC_FIELD | World::HYDROGEN | World::NATURAL_SATELLITES | World::TOXIC_ATMOSPHERE;
    const SATURN_LIKE : i64 = World::JUPITER_LIKE | World::RINGS;

    const ICE_GIANT : i64 = World::HIGH_GRAVITY | World::HIGH_PRESSURE_ATMOSPHERE | World::EXTREME_COLD | World::TOXIC_ATMOSPHERE; // Neptune and Uranus

    const BIOLOGICAL_GAS_GIANT : i64 = World::INSIDE_HABITABLE_ZONE | World::OXYGENATION | World::HIGH_GRAVITY | World::HYDROGEN | World::HIGH_PRESSURE_ATMOSPHERE | World::NATURAL_ANIMAL_BIOLOGY | World::TOLDERABLE_DISASTERS | World::NATURAL_SATELLITES;



    fn build_world (name : String, mass : f64, industries : Vec<Industry>, population : i128, supports : i64) -> World {
        World{ name, mass, industries, population, supports}
    }    
}


struct StarmapLocation {
    x : f32,
    y : f32
}

impl StarmapLocation{
    fn build_starmap_location(x : f32, y : f32) -> StarmapLocation{
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
            
            x = rng.gen_range(0.0 .. 100000.0);
            y = rng.gen_range(0.0 .. 100000.0);


            for star in systems {
                overlap = StarmapLocation::find_distance_to_point(x, y, &star.location) < StarmapLocation::MINIMUM_DISTANCE;
                
                if overlap {
                    break
                }
            }
    
        }

        StarmapLocation::build_starmap_location(x, y)
    }    
}

struct System {
    name : String,
    location : StarmapLocation,
    gdp : i64,
    star_type : i64,
    worlds : Vec<World>,
    space_materials : f64,
    police_presence : f32,
    pirate_presence : f32
}

impl System {
    fn build_system(name : String, location : StarmapLocation, gdp : i64, star_type : i64, worlds : Vec<World>, space_materials : f64, police_presence : f32, pirate_presence : f32) -> System{
        System{name, location, gdp, star_type, worlds, space_materials, police_presence, pirate_presence}
    }    



    fn build_random_system(gs : &GameplayState) -> System {
        let location : StarmapLocation = StarmapLocation::build_random_starmap_location(&gs.systems);

        // these random numbers definitely have some guess work involved, but they are educated guesses based on a short paper by Glenn LeDrew
        let star_type : i64 = star_calcs::StarCalc::new_random_star_type();
        let system_mass : f64 = star_calcs::StarCalc::get_random_system_mass(star_type);
        let planet_mass : f64 = star_calcs::StarCalc::get_planet_mass(system_mass);
        
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        let pirate_presence : f32;
        let police_presence : f32;

        match gs.difficulty {
            DifficultyLevel::Easy => {pirate_presence = rng.gen_range(0.0..0.05); police_presence = rng.gen_range(0.9..1.0)},
            DifficultyLevel::Medium => {pirate_presence = rng.gen_range(0.0..0.10); police_presence = rng.gen_range(0.85..0.95)},
            DifficultyLevel::Hard => {pirate_presence = rng.gen_range(0.05..0.15); police_presence = rng.gen_range(0.75..0.90)},
            DifficultyLevel::Impossible => {pirate_presence = rng.gen_range(0.1..0.25); police_presence = rng.gen_range(0.70..0.80)},
        }

        
        System::build_system(String::from("Test"), location, 10000000, star_type, Vec::new(), system_mass, police_presence, pirate_presence)
    }
}

// Every time we need the gameplay state to make a decision in a new way, this struct needs to change to encorporate that type of task, using a new task of its own.
// Don't currently have any, but that is going to need to change to have any chance of finishing this monstrosity.
struct TaskStack{
    fish : i16
}

fn build_task_stack(fish : i16) -> TaskStack{
    TaskStack{fish}
}


// Every time we need the gameplay stack to actually change, this struct needs to have a new result added to it. This stack being empty is the trigger for copying relevant object orbits to the renderer.
// Lol, this is going to take forever.
struct ResultStack{
    fish : i16
}

fn build_result_stack(fish : i16) -> ResultStack{
    ResultStack {fish}
}

// gameplay state and the gameplay tasks really need their own file.
pub struct GameplayState{
    ship_stats: HashMap<String, ShipStats>,
    ship_: Vec<Ship>, // Only a few ships will be here at any point, because we have RNG for generating random encounters instead.

    weapon_stats: HashMap<String, WeaponStats>,
    
    difficulty : DifficultyLevel,

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

    fn count_star_type(&self, type_ : i64) -> i32{
        let mut count: i32 = 0;

        for system in &self.systems{
            if type_ == system.star_type{
                count += 1;
            }
        }

        count
    }

    pub fn build_gameplay_state(player_name: String, difficulty: DifficultyLevel) -> GameplayState {

        let credits:i64;
    
        match difficulty{
            DifficultyLevel::Easy => credits = 10000,
            DifficultyLevel::Medium => credits = 5000,
            DifficultyLevel::Hard => credits = 3000,
            DifficultyLevel::Impossible => credits = 1500,
            //_=> println!("Unhandled type in build_gameplay_state")  // This triggers a warning....
        }
    
        let sim_time  = 0;
    
        let mut state = GameplayState { ship_stats: HashMap::new(), ship_: Vec::new(), weapon_stats: HashMap::new(), difficulty, systems: Vec::new(), player: build_player(player_name, credits), sim_time, tasks: HashMap::new(), results : build_result_stack(0), multiplayer_stack : HashMap::new()};

        let sol_system = System::build_system(String::from("Sol"), StarmapLocation::build_starmap_location(500.0, 500.0), 100000000000000, star_calcs::StarTypes::F as i64, Vec::new(), 0.0, 100.0, 5.0);

        state.systems.push(sol_system);

        while state.systems.len() < 1000{
            state.add_random_system();
        }
        
        

        state
    
    }
}

