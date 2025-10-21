






mod small_grid_thrusters {
    pub const SG_LG_ATMO_THRUST: f64 = 576_000.0;
    pub const SG_SM_ATMO_THRUST: f64 = 96_000.0;

    pub const SG_SM_FLAT_ATMO_THRUST: f64 = 32_000.0;
    pub const SG_LG_FLAT_ATMO_THRUST: f64 = 230_000.0;

    pub const SG_SM_HYDROGEN_THRUST: f64 = 98_400.0;
    pub const SG_LG_HYDROGEN_THRUST: f64 = 803_340.0;

    pub const SG_SM_ION_THRUST: f64 = 14_400.0;
    pub const SG_LG_ION_THRUST: f64 = 172_800.0;

    pub const SG_SM_PROTOTECH_THRUST: f64 = 561_600.0;
}



enum PlanetsGravity {
    Earth,
    Moon,
    Mars,
    Europa,
    Alien,
    Titan,
    Triton,
    Pertam,
}

impl PlanetsGravity {
    fn gravity(&self) -> f64 {
        let gravity = match self {
            PlanetsGravity::Earth => 9.81,
            PlanetsGravity::Moon => 0.25,
            PlanetsGravity::Mars => 0.9,
            PlanetsGravity::Europa => 0.25,
            PlanetsGravity::Alien => 1.1,
            PlanetsGravity::Titan => 0.25,
            PlanetsGravity::Triton => 1.0,
            PlanetsGravity::Pertam => 1.2,
        };
        gravity
    }
}

fn lift(input_planet: PlanetsGravity, force: f64) -> f64 {
    force/input_planet.gravity()
}

fn main() {
    let force = small_grid_thrusters::SG_LG_ATMO_THRUST * 1.0;

    println!("lift of one sg lg atmo thruster on earth: {} ", lift(PlanetsGravity::Earth, force));
}

