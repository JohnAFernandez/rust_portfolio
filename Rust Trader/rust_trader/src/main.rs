use std::collections::HashMap;
use std::isize;
use std::ops;
use std::num;
use rand::Rng;
use sqlite;

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

fn build_weaponstats(name : String, size : f32, power_required : f32, base_damage : f32, type_ : WeaponTypes) -> WeaponStats {
    WeaponStats { name, size, power_required, base_damage, type_ }
}


// Are these going to do anything yet?  I'm not sure.
enum StarTypes {
    // Giant Stars
    GA,
    GF,
    GG,
    GK,
    GM,

    // Main Sequence
    O,
    B,
    A,
    F,
    G,
    K,
    M,

    // Black Holes
    BH,

    // Nutron Stars
    NS,

    // White Dwarfs
    WB,
    WA,
    WF,
    WG,
    WK,

    // ~ Brown Dwarfs (Simplified)
    L,
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

struct World {
    name : String,
    mass : f64,
    industries : Vec<Industry>,
    population : i128,
    supports : i32 // What is earthlike on this planet from the WorldCharacterists Enum
    
}

impl World{
    fn build_World (name : String, mass : f64, industries : Vec<Industry>, population : i128, supports : i32) -> World {
        World{ name, mass, industries, population, supports}
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
            
            x = rng.gen_range(0.0 .. 100000.0);
            y = rng.gen_range(0.0 .. 100000.0);


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
    name : String,
    location : StarmapLocation,
    gdp : i64,
    star_type : isize,
    worlds : Vec<World>,
    space_materials : f64,
    police_presence : f32,
    pirate_presence : f32
}

impl System {
    fn build_system(name : String, location : StarmapLocation, gdp : i64, star_type : isize, worlds : Vec<World>, space_materials : f64, police_presence : f32, pirate_presence : f32) -> System{
        System{name, location, gdp, star_type, worlds, space_materials, police_presence, pirate_presence}
    }    

    // The chance of each star appearing
    const GA_MAX : f32 = 0.000015;
    const GF_MAX : f32 = 0.0001658;
    const GG_MAX : f32 = 0.0006181;
    const GK_MAX : f32 = 0.00212567;
    const GM_MAX : f32 = 0.002201;
    const O_MAX : f32 = 0.002201;
    const B_MAX : f32 = 0.00250257;
    const A_MAX : f32 = 0.00401013;
    const F_MAX : f32 = 0.01154797;
    const G_MAX : f32 = 0.031146;
    const K_MAX : f32 = 0.0612977;
    const M_MAX : f32 = 0.25125;
    const BH_MAX : f32 = 0.37185;
    const NS_MAX : f32 = 0.41256;
    const WB_MAX : f32 = 0.432159;
    const WA_MAX : f32 = 0.46231;
    const WF_MAX : f32 = 0.47738;
    const WG_MAX : f32 = 0.49246;
    const WK_MAX : f32 = 0.5;
    const L_MAX : f32 = 1.0; // To simplify things, brown dwarfs are about half as likely

    const WORLDS_TO_SUN_MASS_RATIO : f32 = 0.0015;
    const SOLAR_MASSES_TO_KG : f64 = 2000000000000000000000000000000.0;

    const GA_MASS : f32 = 1.75;
    const GF_MASS : f32 = 1.2;
    const GG_MASS : f32 = 1.0;
    const GK_MASS : f32 = 1.0;
    const GM_MASS : f32 = 1.0;
    const O_MASS : f32 = 18.0;
    const B_MASS : f32 = 9.0;
    const A_MASS : f32 = 1.75;
    const F_MASS : f32 = 1.2;
    const G_MASS : f32 = 0.9;
    const K_MASS : f32 = 0.625;
    const M_MASS : f32 = 0.26;
    const BH_MASS : f32 = 71.0;
    const NS_MASS : f32 = 1.6;
    const WB_MASS : f32 = 0.75;
    const WA_MASS : f32 = 0.75;
    const WF_MASS : f32 = 0.75;
    const WG_MASS : f32 = 0.75;
    const WK_MASS : f32 = 0.75;
    const L_MASS : f32 = 0.1;

    fn build_random_system(gs : &GameplayState) -> System {
        let location : StarmapLocation = StarmapLocation::build_random_starmap_location(&gs.systems);


        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let star_rand: f32 = rng.gen();
        
        // these random numbers definitely have some guess work involved, but they are educated guesses based on a short paper by Glenn LeDrew
        let mut star_type : isize = StarTypes::BH as isize;

        match star_rand{
            s if 0.0 <= s && s < System::GA_MAX => star_type = StarTypes::GA as isize,
            s if System::GA_MAX < s && s <= System::GF_MAX => star_type = StarTypes::GF as isize,
            s if System::GF_MAX < s && s <= System::GG_MAX => star_type = StarTypes::GG as isize,
            s if System::GG_MAX < s && s <= System::GK_MAX => star_type = StarTypes::GK as isize,                        
            s if System::GK_MAX < s && s <= System::GM_MAX => star_type = StarTypes::GM as isize,                        
            s if System::GM_MAX < s && s <= System::O_MAX => star_type = StarTypes::O as isize,
            s if System::O_MAX < s && s <= System::B_MAX => star_type = StarTypes::B as isize,                        
            s if System::B_MAX < s && s <= System::A_MAX => star_type = StarTypes::A as isize,                        
            s if System::A_MAX < s && s <= System::F_MAX => star_type = StarTypes::F as isize,                        
            s if System::F_MAX < s && s <= System::G_MAX => star_type = StarTypes::G as isize,
            s if System::G_MAX < s && s <= System::K_MAX => star_type = StarTypes::K as isize,
            s if System::K_MAX < s && s <= System::M_MAX => star_type = StarTypes::M as isize,
            s if System::M_MAX < s && s <= System::BH_MAX => star_type = StarTypes::BH as isize,                        
            s if System::BH_MAX < s && s <= System::NS_MAX => star_type = StarTypes::NS as isize,
            s if System::NS_MAX < s && s <= System::WB_MAX => star_type = StarTypes::WB as isize,
            s if System::WB_MAX < s && s <= System::WA_MAX => star_type = StarTypes::WA as isize,
            s if System::WA_MAX < s && s <= System::WF_MAX => star_type = StarTypes::WF as isize,
            s if System::WF_MAX < s && s <= System::WG_MAX => star_type = StarTypes::WG as isize,
            s if System::WG_MAX < s && s <= System::WK_MAX => star_type = StarTypes::WK as isize,
            s if System::WK_MAX < s && s <= System::L_MAX => star_type = StarTypes::L as isize,                        

            _=> println!("Bad random number {} generated for star type pick.", star_rand)
        }

        let mut system_mass : f64 = 0.0;

        match star_type{
            s if s == StarTypes::GA as isize => system_mass = (System::GA_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64,
            s if s == StarTypes::GF as isize => system_mass = (System::GF_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64,
            s if s == StarTypes::GG as isize => system_mass = (System::GG_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64,
            s if s == StarTypes::GK as isize => system_mass = (System::GK_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64,                        
            s if s == StarTypes::GM as isize => system_mass = (System::GM_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64,                        
            s if s == StarTypes::O as isize => system_mass = (System::O_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::B as isize => system_mass = (System::B_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64,                        
            s if s == StarTypes::A as isize => system_mass = (System::A_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64,                         
            s if s == StarTypes::F as isize => system_mass = (System::F_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::G as isize => system_mass = (System::G_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::K as isize => system_mass = (System::K_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::M as isize => system_mass = (System::M_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64,                         
            s if s == StarTypes::BH as isize => system_mass = (System::BH_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::NS as isize => system_mass = (System::NS_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::WB as isize => system_mass = (System::WB_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::WA as isize => system_mass = (System::WA_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::WF as isize => system_mass = (System::WF_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::WG as isize => system_mass = (System::WG_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::WK as isize => system_mass = (System::WK_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 
            s if s == StarTypes::L as isize => system_mass = (System::L_MASS * System::WORLDS_TO_SUN_MASS_RATIO) as f64, 

            _=> println!("Bad star type of {} found in mass calc.", star_type)
        }

        let pirate_presence : f32;
        let police_presence : f32;

        match gs.difficulty {
            DifficultyLevel::Easy => {pirate_presence = rng.gen_range(0.0..0.05); police_presence = rng.gen_range(0.9..1.0)},
            DifficultyLevel::Medium => {pirate_presence = rng.gen_range(0.0..0.10); police_presence = rng.gen_range(0.85..0.95)},
            DifficultyLevel::Hard => {pirate_presence = rng.gen_range(0.05..0.15); police_presence = rng.gen_range(0.75..0.90)},
            DifficultyLevel::Impossible => {pirate_presence = rng.gen_range(0.1..0.25); police_presence = rng.gen_range(0.70..0.80)},
        }

        match star_type {
            

            _=> ()
        }

        
        system_mass *= System::SOLAR_MASSES_TO_KG;        

        System::build_system(String::from("Test"), location, 10000000, star_type, Vec::new(), system_mass, police_presence, pirate_presence)
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

    fn count_star_type(&self, type_ : isize) -> i32{
        let mut count: i32 = 0;

        for system in &self.systems{
            if type_ == system.star_type{
                count += 1;
            }
        }

        count
    }
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

    let sim_time  = 0;

    GameplayState { ship_stats: HashMap::new(), ship_: Vec::new(), weapon_stats: HashMap::new(), difficulty, systems: Vec::new(), player: build_player(player_name, credits), sim_time, tasks: HashMap::new(), results : build_ResultStack(0), multiplayer_stack : HashMap::new() }
}

fn start_game() {
    let mut state: GameplayState = build_gameplaystate(String::from("Test Player Name"), DifficultyLevel::Easy);

    let mut debug_counter = 0;

    while state.systems.len() < 1000{
        state.add_random_system();
        debug_counter +=1;

        if state.systems.len() != debug_counter {
            println!("We have a problem. System size {} debug counter {}", state.systems.len(), debug_counter);
        }
    }

    println!("Showing generated systems.");

    for star_type in 0..20{
        println!("Type {} has {} in the gamestate", star_type, state.count_star_type(star_type));
    }
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

    const types : [&str ; 6] = ["Fighter", "Destroyer", "Frigate", "Cruiser", "Battleship", "Carrier"];

    fn build_shipstats(class_name : String, type_ : i8, max_cargo_volume : f32, crew_minimum : i16, crew_maximum : i16, base_mass: f64, sensor_range: f64, weapon_slots: i16, equipment_slots: i16) -> ShipStats {
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



fn main() {
    // init SDL or some other goofiness here.


    // actually start stuff
    start_game()
    // get input for name and difficulty.
    //build_GameplayState(player_name, difficulty)
}
