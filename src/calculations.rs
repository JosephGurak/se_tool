pub enum PlanetsGravity {
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
    pub fn gravity(&self) -> f64 {
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

pub fn lift(input_planet: PlanetsGravity, force: f64) -> f64 {
    force/input_planet.gravity()
}