#![allow(dead_code)] // until more of this is written.

use rand::Rng;

use super::World;

pub struct StarCalc {
}

impl StarCalc {
    // The chance of each star appearing
    const GA_CHANCE_MAX : f32 = 0.000015;
    const GF_CHANCE_MAX : f32 = 0.0001658;
    const GG_CHANCE_MAX : f32 = 0.0006181;
    const GK_CHANCE_MAX : f32 = 0.00212567;
    const GM_CHANCE_MAX : f32 = 0.002201;
    const O_CHANCE_MAX : f32 = 0.002201;
    const B_CHANCE_MAX : f32 = 0.00250257;
    const A_CHANCE_MAX : f32 = 0.00401013;
    const F_CHANCE_MAX : f32 = 0.01154797;
    const G_CHANCE_MAX : f32 = 0.031146;
    const K_CHANCE_MAX : f32 = 0.0612977;
    const M_CHANCE_MAX : f32 = 0.25125;
    const BH_CHANCE_MAX : f32 = 0.37185;
    const NS_CHANCE_MAX : f32 = 0.41256;
    const WB_CHANCE_MAX : f32 = 0.432159;
    const WA_CHANCE_MAX : f32 = 0.46231;
    const WF_CHANCE_MAX : f32 = 0.47738;
    const WG_CHANCE_MAX : f32 = 0.49246;
    const WK_CHANCE_MAX : f32 = 0.5;
    const L_CHANCE_MAX : f32 = 1.0; // To simplify things, brown dwarfs are about half as likely

    const WORLDS_TO_SUN_MASS_RATIO : f64 = 0.0015;
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
    const L_MIN_MASS : f32 = 0.005;

    // this is all based on the solar system.  I don't really have time to make something that looks like the rest of the universe.
    const MASS_OF_EARTH : f64 = 5972200000000000000000000.0; // In KG
    const MASS_OF_MINOR_BODIES_RATIO : f64 = 0.0000015; // You read that right, very little asteroids, comets, iceball mass actually exist in a planetary system
    const MASS_OF_TERRESTRIAL_PLANETS_RATIO : f64 = 0.00443; // Still comparatively small 
    const MASS_OF_ICE_GIANTS_RATIO : f64 = 0.0708; // Still pretty small
    const MASS_OF_GAS_GIANTS_RATIO : f64 = 1.0 - StarCalc::MASS_OF_MINOR_BODIES_RATIO - StarCalc::MASS_OF_TERRESTRIAL_PLANETS_RATIO - StarCalc::MASS_OF_ICE_GIANTS_RATIO;

    // in earth masses
    const MIN_GAS_GIANT : f64 = 3.0; // These are based on research
    const MAX_GAS_GIANT : f64 = 3180.0; // These are based on research
    const MIN_ICE_GIANT : f64 = 1.5; // These are kind of arbitrary limits based on my intuition.
    const MAX_ICE_GIANT : f64 = 7.0; // These are kind of arbitrary limits based on my intuition.

    pub fn new_random_star_type() -> i64 {
        let mut star_type : i64 = StarTypes::BH as i64;

        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let star_rand: f32 = rng.gen();

        match star_rand {
            s if 0.0 <= s && s < StarCalc::GA_CHANCE_MAX => star_type = StarTypes::GA as i64,
            s if StarCalc::GA_CHANCE_MAX < s && s <= StarCalc::GF_CHANCE_MAX => star_type = StarTypes::GF as i64,
            s if StarCalc::GF_CHANCE_MAX < s && s <= StarCalc::GG_CHANCE_MAX => star_type = StarTypes::GG as i64,
            s if StarCalc::GG_CHANCE_MAX < s && s <= StarCalc::GK_CHANCE_MAX => star_type = StarTypes::GK as i64,                        
            s if StarCalc::GK_CHANCE_MAX < s && s <= StarCalc::GM_CHANCE_MAX => star_type = StarTypes::GM as i64,                        
            s if StarCalc::GM_CHANCE_MAX < s && s <= StarCalc::O_CHANCE_MAX => star_type = StarTypes::O as i64,
            s if StarCalc::O_CHANCE_MAX < s && s <= StarCalc::B_CHANCE_MAX => star_type = StarTypes::B as i64,                        
            s if StarCalc::B_CHANCE_MAX < s && s <= StarCalc::A_CHANCE_MAX => star_type = StarTypes::A as i64,                        
            s if StarCalc::A_CHANCE_MAX < s && s <= StarCalc::F_CHANCE_MAX => star_type = StarTypes::F as i64,                        
            s if StarCalc::F_CHANCE_MAX < s && s <= StarCalc::G_CHANCE_MAX => star_type = StarTypes::G as i64,
            s if StarCalc::G_CHANCE_MAX < s && s <= StarCalc::K_CHANCE_MAX => star_type = StarTypes::K as i64,
            s if StarCalc::K_CHANCE_MAX < s && s <= StarCalc::M_CHANCE_MAX => star_type = StarTypes::M as i64,
            s if StarCalc::M_CHANCE_MAX < s && s <= StarCalc::BH_CHANCE_MAX => star_type = StarTypes::BH as i64,                        
            s if StarCalc::BH_CHANCE_MAX < s && s <= StarCalc::NS_CHANCE_MAX => star_type = StarTypes::NS as i64,
            s if StarCalc::NS_CHANCE_MAX < s && s <= StarCalc::WB_CHANCE_MAX => star_type = StarTypes::WB as i64,
            s if StarCalc::WB_CHANCE_MAX < s && s <= StarCalc::WA_CHANCE_MAX => star_type = StarTypes::WA as i64,
            s if StarCalc::WA_CHANCE_MAX < s && s <= StarCalc::WF_CHANCE_MAX => star_type = StarTypes::WF as i64,
            s if StarCalc::WF_CHANCE_MAX < s && s <= StarCalc::WG_CHANCE_MAX => star_type = StarTypes::WG as i64,
            s if StarCalc::WG_CHANCE_MAX < s && s <= StarCalc::WK_CHANCE_MAX => star_type = StarTypes::WK as i64,
            s if StarCalc::WK_CHANCE_MAX < s && s <= StarCalc::L_CHANCE_MAX => star_type = StarTypes::L as i64,                        

            _=> println!("Bad random number {} generated for star type pick.", star_rand)
        }

        star_type
    }

    pub fn get_random_system_mass(star_type : i64) -> f64{

        let mut max_system_mass : f64 = 0.0;
        let mut min_system_mass : f64 = 0.0;

        match star_type{
            s if s == StarTypes::GA as i64 => {max_system_mass = (StarCalc::GA_MASS) as f64; min_system_mass = StarCalc::GF_MASS as f64},
            s if s == StarTypes::GF as i64 => {max_system_mass = (StarCalc::GF_MASS) as f64; min_system_mass = StarCalc::GG_MASS as f64},
            s if s == StarTypes::GG as i64 => {max_system_mass = (StarCalc::GG_MASS) as f64; min_system_mass = StarCalc::GK_MASS as f64},
            s if s == StarTypes::GK as i64 => {max_system_mass = (StarCalc::GK_MASS) as f64; min_system_mass = StarCalc::GM_MASS as f64},                        
            s if s == StarTypes::GM as i64 => {max_system_mass = (StarCalc::GM_MASS) as f64; min_system_mass = StarCalc::L_MASS as f64},                        
            s if s == StarTypes::O as i64 => {max_system_mass = (StarCalc::O_MASS) as f64; min_system_mass = StarCalc::B_MASS as f64}, 
            s if s == StarTypes::B as i64 => {max_system_mass = (StarCalc::B_MASS) as f64; min_system_mass = StarCalc::A_MASS as f64},                        
            s if s == StarTypes::A as i64 => {max_system_mass = (StarCalc::A_MASS) as f64; min_system_mass = StarCalc::F_MASS as f64},                         
            s if s == StarTypes::F as i64 => {max_system_mass = (StarCalc::F_MASS) as f64; min_system_mass = StarCalc::G_MASS as f64}, 
            s if s == StarTypes::G as i64 => {max_system_mass = (StarCalc::G_MASS) as f64; min_system_mass = StarCalc::K_MASS as f64}, 
            s if s == StarTypes::K as i64 => {max_system_mass = (StarCalc::K_MASS) as f64; min_system_mass = StarCalc::M_MASS as f64}, 
            s if s == StarTypes::M as i64 => {max_system_mass = (StarCalc::M_MASS) as f64; min_system_mass = StarCalc::L_MASS as f64},                         
            s if s == StarTypes::BH as i64 => {max_system_mass = (StarCalc::BH_MASS) as f64; min_system_mass = StarCalc::NS_MASS as f64}, 
            s if s == StarTypes::NS as i64 => {max_system_mass = (StarCalc::NS_MASS) as f64; min_system_mass = StarCalc::WB_MASS as f64}, 
            s if s == StarTypes::WB as i64 => {max_system_mass = (StarCalc::WB_MASS) as f64; min_system_mass = StarCalc::WA_MASS as f64}, 
            s if s == StarTypes::WA as i64 => {max_system_mass = (StarCalc::WA_MASS) as f64; min_system_mass = StarCalc::WF_MASS as f64}, 
            s if s == StarTypes::WF as i64 => {max_system_mass = (StarCalc::WF_MASS) as f64; min_system_mass = StarCalc::WG_MASS as f64}, 
            s if s == StarTypes::WG as i64 => {max_system_mass = (StarCalc::WG_MASS) as f64; min_system_mass = StarCalc::WK_MASS as f64}, 
            s if s == StarTypes::WK as i64 => {max_system_mass = (StarCalc::WK_MASS) as f64; min_system_mass = StarCalc::L_MASS as f64}, 
            s if s == StarTypes::L as i64 => {max_system_mass = (StarCalc::L_MASS) as f64; min_system_mass = StarCalc::L_MIN_MASS as f64}, 

            _=> println!("Bad star type of {} found in mass calc.", star_type)
        }

        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let mass_rand: f64 = rng.gen();

        min_system_mass + ((max_system_mass - min_system_mass) * mass_rand)
    }

    // remember, returns the mass of non-star objects in KG
    pub fn get_planet_mass(system_mass : f64) -> f64 {
        system_mass * StarCalc::WORLDS_TO_SUN_MASS_RATIO * StarCalc::SOLAR_MASSES_TO_KG
    }

    pub fn get_gas_giant_mass(planet_mass : f64) -> f64 {
        planet_mass * StarCalc::MASS_OF_GAS_GIANTS_RATIO
    }

    pub fn get_ice_giant_mass(planet_mass : f64) -> f64 {
        planet_mass * StarCalc::MASS_OF_ICE_GIANTS_RATIO
    }

    pub fn get_rocky_mass(planet_mass : f64) -> f64 {
        planet_mass * StarCalc::MASS_OF_TERRESTRIAL_PLANETS_RATIO
    }

    pub fn get_minor_mass(planet_mass : f64) -> f64 {
        planet_mass * StarCalc::MASS_OF_MINOR_BODIES_RATIO
    }

    const MAX_JUPITER_LIKE_CHANCE : f64 = 0.60;
    const MAX_SATURN_LIKE_CHANCE : f64 = 0.95;

    pub fn generate_random_gas_giants(mass : f64, mut worlds : Vec<World>) -> Vec<World>{
        let max_planets: f64 = mass / StarCalc::MIN_GAS_GIANT;


        // this basically enforces 
        if  max_planets < 1.0 {
            worlds.push(World::build_world(String::from("TEST GAS GIANT"), mass, Vec::new(), 0, World::JUPITER_LIKE));
            return worlds;
        }

        let mut planets = 0;
        let mut remaining_mass :f64 = mass;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        while (planets as f64) < max_planets {
            let mass_rand : f64 = rng.gen_range(StarCalc::MIN_GAS_GIANT..StarCalc::MAX_GAS_GIANT);
            remaining_mass -= mass_rand;
            
            let type_rand : f64 = rng.gen();
            let type_flags : i64;

            match type_rand{
                t if t >= 0.0 && t < StarCalc::MAX_JUPITER_LIKE_CHANCE => type_flags = World::JUPITER_LIKE,
                t if t >= StarCalc::MAX_JUPITER_LIKE_CHANCE && t < StarCalc::MAX_SATURN_LIKE_CHANCE => type_flags = World::SATURN_LIKE,
                t if t >= StarCalc::MAX_SATURN_LIKE_CHANCE && t <= 1.0 => type_flags = World::BIOLOGICAL_GAS_GIANT,
                _=> {println!("BAD chance in generate random gas giants of {} defaulting to Jupiter-like", type_rand); type_flags = World::JUPITER_LIKE}
            }

            worlds.push(World::build_world(String::from("TEST GAS GIANT"), mass_rand, Vec::new(), 0, type_flags));

            if remaining_mass < StarCalc::MIN_GAS_GIANT{
                worlds.push(World::build_world(String::from("TEST GAS GIANT"), mass, Vec::new(), 0, World::JUPITER_LIKE));
                break;
            }

            planets += 1;
        }

        worlds
    }

    pub fn generate_random_ice_giants(mass : f64, mut worlds : Vec<World>) -> Vec<World> {
        let max_planets: f64 = mass / StarCalc::MIN_ICE_GIANT;


        // this basically enforces 
        if  max_planets < 1.0 {
            worlds.push(World::build_world(String::from("TEST ICE GIANT"), mass, Vec::new(), 0, World::ICE_GIANT));
            return worlds;
        }

        let mut planets = 0;
        let mut remaining_mass : f64 = mass;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        while (planets as f64) < max_planets {
            let mass_rand: f64 = rng.gen_range(StarCalc::MIN_ICE_GIANT..StarCalc::MAX_ICE_GIANT);
            remaining_mass -= mass_rand;
            
            worlds.push(World::build_world(String::from("TEST ICE GIANT"), mass_rand, Vec::new(), 0, World::ICE_GIANT));

            if remaining_mass < StarCalc::MIN_GAS_GIANT{
                worlds.push(World::build_world(String::from("TEST ICE GIANT"), mass, Vec::new(), 0, World::ICE_GIANT));
                break;
            }

            planets += 1;
        }

        worlds
    }
    


}


// Are these going to do anything yet?  I'm not sure.
pub enum StarTypes {
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