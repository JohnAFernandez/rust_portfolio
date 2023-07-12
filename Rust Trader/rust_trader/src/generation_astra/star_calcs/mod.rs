#![allow(dead_code)] // until more of this is written.

use rand::Rng;
use super::World;

pub struct StarCalc {
}

static mut DEBUG : bool = false;

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
    const BH_MASS : f32 = 20.0; // small stellar black hole.  
    const NS_MASS : f32 = 1.6;
    const WB_MASS : f32 = 0.75;
    const WA_MASS : f32 = 0.75;
    const WF_MASS : f32 = 0.75;
    const WG_MASS : f32 = 0.75;
    const WK_MASS : f32 = 0.75;
    const L_MASS : f32 = 0.1;
    const L_MIN_MASS : f32 = 0.005;

    // Magnitudes are backwards.
    const O_MIN_MAGNITUDE : f32 = -2.0;
    const O_MAX_MAGNITUDE : f32 = -10.0;
    const B_MIN_MAGNITUDE : f32 = 1.0;
    const B_MAX_MAGNITUDE : f32 = -5.0;
    const A_MIN_MAGNITUDE : f32 = 2.75;
    const A_MAX_MAGNITUDE : f32 = -1.75;
    const F_MIN_MAGNITUDE : f32 = 4.25;
    const F_MAX_MAGNITUDE : f32 = 0.0;
    const G_MIN_MAGNITUDE : f32 = 8.25;
    const G_MAX_MAGNITUDE : f32 = 1.75;
    const K_MIN_MAGNITUDE : f32 = 10.75;
    const K_MAX_MAGNITUDE : f32 = 4.5;
    const M_MIN_MAGNITUDE : f32 = 16.0;
    const M_MAX_MAGNITUDE : f32 = 8.5;

    // this is all based on the solar system.  I don't really have time to make something that looks like the rest of the universe.
    pub const MASS_OF_EARTH : f64 = 5972200000000000000000000.0; // In KG, 5.9722 * 10 ^ 24 KG
    const MASS_OF_MINOR_BODIES_RATIO : f64 = 0.0001; // You read that right, very little asteroids, comets, iceball mass actually exist in a planetary system
    const MASS_OF_TERRESTRIAL_PLANETS_RATIO : f64 = 0.01443; // Still comparatively small 
    const MASS_OF_ICE_GIANTS_RATIO : f64 = 0.0708; // Still pretty small
    const MASS_OF_GAS_GIANTS_RATIO : f64 = 1.0 - StarCalc::MASS_OF_MINOR_BODIES_RATIO - StarCalc::MASS_OF_TERRESTRIAL_PLANETS_RATIO - StarCalc::MASS_OF_ICE_GIANTS_RATIO;

    // in earth masses
    const MIN_GAS_GIANT_MASS : f64 = 3.0 * StarCalc::MASS_OF_EARTH; // These are based on quick research
    const MAX_GAS_GIANT_MASS : f64 = 3180.0 * StarCalc::MASS_OF_EARTH; // These are based on quick research
    const MIN_ICE_GIANT_MASS : f64 = 5.0 * StarCalc::MASS_OF_EARTH; // These are kind of arbitrary limits based on my intuition.
    const MAX_ICE_GIANT_MASS : f64 = 30.0 * StarCalc::MASS_OF_EARTH; // These are kind of arbitrary limits based on my intuition.
    const MIN_TERRESTRIAL_PLANET : f64 = 0.005 * StarCalc::MASS_OF_EARTH; // Extremely arbitrary
    const MAX_SUPER_EARTH_MASS : f64 = 10.0 * StarCalc::MASS_OF_EARTH; // Biggest super earth found so far is about this size.
    const MIN_MINOR_BODY_MASS : f64 = 100000000000000000000.0; // 1.0 * 10 ^20 KG
    const MAX_MINOR_BODY_MASS : f64 = 20000000000000000000000.0; // 2.0*10^22 KG


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

    const MAX_JUPITER_LIKE_CHANCE : f64 = 0.70;

    pub fn generate_random_gas_giants(mass : f64, mut worlds : Vec<World>) -> Vec<World>{
        let max_planets: f64 = mass / (StarCalc::MIN_GAS_GIANT_MASS);

        // enforce minimum planet size.
        if  max_planets < 1.0 {
            return worlds;
        }

        let mut planets = 0;
        let mut remaining_mass :f64 = mass;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        while (planets as f64) < max_planets {
            let mass_rand: f64 = f64::powf(rng.gen_range(1.0..2.0), 11.6348110502) * StarCalc::MIN_GAS_GIANT_MASS;
            remaining_mass -= mass_rand;
            
            let type_rand : f64 = rng.gen();
            let type_flags : i64;

            match type_rand{
                t if t >= 0.0 && t < StarCalc::MAX_JUPITER_LIKE_CHANCE => type_flags = World::JUPITER_LIKE,
                t if t >= StarCalc::MAX_JUPITER_LIKE_CHANCE && t <= 1.0 => type_flags = World::SATURN_LIKE,
                _=> {println!("BAD chance in generate random gas giants of {} defaulting to Jupiter-like", type_rand); type_flags = World::JUPITER_LIKE}
            }

            worlds.push(World::build_world(String::from("TEST GAS GIANT"), mass_rand, Vec::new(), 0, type_flags));

            if remaining_mass < StarCalc::MIN_GAS_GIANT_MASS{
                break;
            }

            planets += 1;
        }

        worlds
    }

    pub fn generate_random_ice_giants(mass : f64, mut worlds : Vec<World>) -> Vec<World> {
        let max_planets: f64 = mass / StarCalc::MIN_ICE_GIANT_MASS;

        // this basically enforces minimum planet size
        if  max_planets < 1.0 {
            return worlds;
        }

        let mut planets = 0;
        let mut remaining_mass : f64 = mass;
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        while (planets as f64) < max_planets {
            let mass_rand: f64 = (f64::powf(rng.gen_range(1.0..2.0), 4.6438561898)) * StarCalc::MIN_ICE_GIANT_MASS;
            remaining_mass -= mass_rand;
            
            worlds.push(World::build_world(String::from("TEST ICE GIANT"), mass_rand, Vec::new(), 0, World::ICE_GIANT));

            if remaining_mass < StarCalc::MIN_GAS_GIANT_MASS{
                break;
            }

            planets += 1;
        }

        worlds
    }
    
    pub fn generate_random_rocky_planets(planet_mass : f64, mut worlds : Vec<World>) -> Vec<World> {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let num_worlds : i32 = rng.gen_range(0 .. 7);
        let mut remaining_mass = planet_mass;
        let mut x : i32 = 0;

        while x < num_worlds && remaining_mass > 0.0 {
            // this circumvents having to write a long match statement, since this gives a max of 10, the largest known super earth 
            let mut rand_mass : f64 = f64::powf(0.5 + rng.gen::<f64>(), 5.6788735873) * StarCalc::MASS_OF_EARTH;

            // clamp our mass.
            if rand_mass >  StarCalc::MAX_SUPER_EARTH_MASS {
                rand_mass = StarCalc::MAX_SUPER_EARTH_MASS;
            } else if rand_mass < StarCalc::MIN_TERRESTRIAL_PLANET {
                rand_mass = StarCalc::MIN_TERRESTRIAL_PLANET;
            }

            // decide the characteristics of our planet, first with what all these will have.
            let mut planet_flags : i64 = World::RAW_MATERIALS | World::NATURAL_SOIL;

            // Does this have earth like gravity?
            if rand_mass > (0.6 * StarCalc::MASS_OF_EARTH) && rand_mass < (1.2 * StarCalc::MASS_OF_EARTH) {
                planet_flags |= World::EARTH_GRAVITY;
            } else if rand_mass > (1.2 * StarCalc::MASS_OF_EARTH) {
                planet_flags |= World::HIGH_GRAVITY;
            }

            // will this planet have a magnetic field
            if rng.gen::<f32>() < 0.40 {
                planet_flags |= World::MAGNETIC_FIELD;
                planet_flags |= World::TECTONICALLY_ACTIVE;
            }

            // when gravity is low enough, it cannot maintain its atmosphere
            // No I didn't research this number
            if rand_mass < (0.2 * StarCalc::MASS_OF_EARTH) {
                planet_flags |= World::NO_ATMOSPHERE;
            } else {
                // we need to determine what type of atmosphere we have here

                // low mass means alsmost no atmosphere (think mars)
                if rand_mass < (0.5 * StarCalc::MASS_OF_EARTH){                    
                    planet_flags |= World::MINIMAL_ATMOSPHERE;

                // but if we're not too small, we might have ended up with a Venus by chance    
                } else if rng.gen::<f32>() < 0.1 {
                    planet_flags |= World::HIGH_PRESSURE_ATMOSPHERE;
                }

                // worlds with a magnetic field will not have their hydrogen sheered off by solar wind and stuff
                if planet_flags & World::MAGNETIC_FIELD != 0{
                    planet_flags |= World::HYDROGEN;
                }


                // It may have a toxic atmosphere, most worlds do
                if rng.gen::<f32>() < 0.66{
                    planet_flags |= World::TOXIC_ATMOSPHERE;
                    

                    if rng.gen::<f32>() < 0.95 {
                        planet_flags |= World::ACIDIC;
                    } else {
                        planet_flags |= World::ALKALINE;
                    }


                } else {
                    planet_flags |= World::OXYGENATION;
                }

            }

            // does this world have oceans? (No atmosphere means subsurface ocean)
            if rng.gen::<f32>() < 0.1 {
                planet_flags |= World::OCEANS;
                
                // If there's an atmosphere and an ocean, there's a water cycle.
                if planet_flags & World::NO_ATMOSPHERE == 0 {
                    planet_flags |= World::WATER_CYCLE;
                }

                // rare effect where the oceans are toxic to our biology
                if rng.gen::<f32>() < 0.01 {
                    planet_flags |= World::TOXIC_OCEANS;
                }
            }

            if rng.gen::<f32>() < 0.05 {
                planet_flags |= World::HIGH_VOLCANISM;
            }

            // Do we have reasonable disasters?  
            if planet_flags & (World::HIGH_VOLCANISM | World::MINIMAL_ATMOSPHERE | World::NO_ATMOSPHERE) == 0{
                planet_flags |= World::TOLERABLE_DISASTERS;
            }

            // most planets have moons, at least in the solar system.  Even mars does.
            if rng.gen::<f32>() > 0.2 {
                planet_flags |= World::NATURAL_SATELLITES;
            
            }

            if rng.gen::<f32>() < 0.1 {
                planet_flags |= World::RINGS;
            }

            // yes, I know "LIFE CAN BE SO DIFFERENT THAN US", ok but this is probably the easiest way for life to emerge.
            if planet_flags & World::HYDROGEN != 0 && planet_flags & World::WATER_CYCLE != 0 {
                if rng.gen::<f32>() < 0.25{
                    planet_flags |= World::NATURAL_MICROBES;
                    
                    if rng.gen::<f32>() < 0.15 {
                        planet_flags |= World::NATURAL_PLANTS;

                        if rng.gen::<f32>() < 0.50 {
                            planet_flags |= World::NATURAL_ANIMAL_BIOLOGY;

                            if rng.gen::<f32>() < 0.50 {
                                planet_flags |= World::NATURAL_CIV;
                            }
                        }
                    }
                } 
            }

            remaining_mass -= rand_mass;

            worlds.push(World::build_world(String::from("TEST Terrestrial"), rand_mass, Vec::new(), 0, planet_flags));

            if remaining_mass < 0.1 * StarCalc::MASS_OF_EARTH{
                break;
            }

            x += 1;
        }

        worlds
    }

    pub fn generate_random_minor_planets(planet_mass : f64, mut worlds : Vec<World>) -> Vec<World> {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let num_worlds : i32 = rng.gen_range(10 .. 40);         
        let mut x = 0;
        let mut remaining_mass = planet_mass;

        while x < num_worlds && remaining_mass > 0.0 {
            let mut rand_mass : f64 = (rng.gen::<f64>().powf(3.0) * (StarCalc::MAX_MINOR_BODY_MASS - StarCalc::MIN_MINOR_BODY_MASS)) + StarCalc::MIN_MINOR_BODY_MASS;

            // clamp
            if rand_mass < StarCalc::MIN_MINOR_BODY_MASS {
                rand_mass = StarCalc::MIN_MINOR_BODY_MASS;
            } else if rand_mass > StarCalc::MAX_MINOR_BODY_MASS {
                rand_mass = StarCalc::MAX_MINOR_BODY_MASS;
            }

            let mut planet_flags = World::PLUTO_LIKE;
            
            if rand_mass < ((StarCalc::MAX_MINOR_BODY_MASS - StarCalc::MIN_MINOR_BODY_MASS) / 2.0 ) + StarCalc::MIN_MINOR_BODY_MASS {
                planet_flags |= World::SPHEROID;
            }

            worlds.push(World::build_world(String::from("TEST Minor"), rand_mass, Vec::new(), 0, planet_flags));

            remaining_mass -= rand_mass;
            x += 1;
        }

        worlds
    }

    // based on a formula found online at plantearybiology.com
    pub fn habitable_range(star_type : i64) -> (f32,f32) {

        let conversion_value: f32; 
        let max_magnitude: f32;
        let min_magnitude: f32;

        match star_type {
            s if s == StarTypes::GA as i64 => {max_magnitude = -10.0; min_magnitude = -10.0; conversion_value = 0.0},
            s if s == StarTypes::GF as i64 => {max_magnitude = -10.0; min_magnitude = -10.0; conversion_value = 0.0},
            s if s == StarTypes::GG as i64 => {max_magnitude = -10.0; min_magnitude = -10.0; conversion_value = 0.0},
            s if s == StarTypes::GK as i64 => {max_magnitude = -10.0; min_magnitude = -10.0; conversion_value = 0.0},                        
            s if s == StarTypes::GM as i64 => {max_magnitude = -10.0; min_magnitude = -10.0; conversion_value = 0.0},                        
            s if s == StarTypes::O as i64 => {max_magnitude = -10.0; min_magnitude = -10.0; conversion_value = 0.0}, 
            s if s == StarTypes::B as i64 => {max_magnitude = StarCalc::B_MAX_MAGNITUDE; min_magnitude = StarCalc::B_MIN_MAGNITUDE; conversion_value = -2.0},                        
            s if s == StarTypes::A as i64 => {max_magnitude = StarCalc::A_MAX_MAGNITUDE; min_magnitude = StarCalc::A_MIN_MAGNITUDE; conversion_value = -0.3},                         
            s if s == StarTypes::F as i64 => {max_magnitude = StarCalc::F_MAX_MAGNITUDE; min_magnitude = StarCalc::F_MIN_MAGNITUDE; conversion_value = -0.15}, 
            s if s == StarTypes::G as i64 => {max_magnitude = StarCalc::G_MAX_MAGNITUDE; min_magnitude = StarCalc::G_MIN_MAGNITUDE; conversion_value = -0.4}, 
            s if s == StarTypes::K as i64 => {max_magnitude = StarCalc::K_MAX_MAGNITUDE; min_magnitude = StarCalc::K_MIN_MAGNITUDE; conversion_value = -0.8}, 
            s if s == StarTypes::M as i64 => {max_magnitude = StarCalc::M_MAX_MAGNITUDE; min_magnitude = StarCalc::M_MIN_MAGNITUDE; conversion_value = -2.0},                         
            s if s == StarTypes::BH as i64 => {max_magnitude = 20.0; min_magnitude = 20.0; conversion_value = 0.0}, 
            s if s == StarTypes::NS as i64 => {max_magnitude = StarCalc::M_MAX_MAGNITUDE; min_magnitude = StarCalc::M_MIN_MAGNITUDE; conversion_value = 0.0}, 
            s if s == StarTypes::WB as i64 => {max_magnitude = StarCalc::M_MAX_MAGNITUDE; min_magnitude = StarCalc::M_MIN_MAGNITUDE; conversion_value = 0.0}, 
            s if s == StarTypes::WA as i64 => {max_magnitude = StarCalc::M_MAX_MAGNITUDE; min_magnitude = StarCalc::M_MIN_MAGNITUDE; conversion_value = 0.0}, 
            s if s == StarTypes::WF as i64 => {max_magnitude = StarCalc::M_MAX_MAGNITUDE; min_magnitude = StarCalc::M_MIN_MAGNITUDE; conversion_value = 0.0}, 
            s if s == StarTypes::WG as i64 => {max_magnitude = StarCalc::M_MAX_MAGNITUDE; min_magnitude = StarCalc::M_MIN_MAGNITUDE; conversion_value = 0.0}, 
            s if s == StarTypes::WK as i64 => {max_magnitude = StarCalc::M_MAX_MAGNITUDE; min_magnitude = StarCalc::M_MIN_MAGNITUDE; conversion_value = 0.0}, 
            s if s == StarTypes::L as i64 => {max_magnitude = 20.0; min_magnitude = 20.0; conversion_value = 0.0}, 
            _=> {println!("Unknown star type in habitable min"); max_magnitude = 20.0; min_magnitude = 20.0; conversion_value = 0.0 }
        }

        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        let rand_mag : f32 = if max_magnitude == min_magnitude { 20.0 } else { rng.gen_range(max_magnitude..min_magnitude) };

        // forumla is Absolute Magnitude (rand_mag) - table value - 4.72, divided by -2.5
        // raise 10 to that value.  Then the inner is done by dividng that by 1.1 and then
        // taking the square root.  And the outer is done by dividn that by 0.53 and then
        // then again taking the square root.
        const SUBTRACT_FACTOR: f32 = -4.72;
        const POWER_BASE: f32 = 10.0;
        const DIVISION_VALUE: f32 = -2.5;

        let intermediate_value : f32 = POWER_BASE.powf((rand_mag + conversion_value + SUBTRACT_FACTOR) / DIVISION_VALUE); 

        const INNER_DIVIDE:f32 = 1.1;
        const OUTER_DIVIDE:f32 = 0.53;

        ((intermediate_value / INNER_DIVIDE).sqrt(), (intermediate_value / OUTER_DIVIDE).sqrt())
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