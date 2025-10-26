
// these are values for blocks that are constant 

// values here represented in newtons instead of kN
pub mod small_grid_thrusters {
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
