mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
struct FluidConfig {
    n: u16,
    dt: u8,
    diffusion: i32,
}

type PropertyType = Vec<f32>;

#[wasm_bindgen]
pub struct Fluid {
    config: FluidConfig,
    velocity_x: PropertyType,
    velocityY: PropertyType,
    initial_velocity_x: PropertyType,
    initial_velocityY: PropertyType,
    density: PropertyType,
    initial_density: PropertyType,
    size: u16,
}

impl Fluid {
    fn new(config: FluidConfig) -> Fluid {
        let size = (config.n + 2) * (config.n + 2);
        let vector_size = size.into();
        Fluid {
            config,
            velocity_x: vec![0.0; vector_size],
            velocityY: vec![0.0; vector_size],
            initial_velocity_x: vec![0.0; vector_size],
            initial_velocityY: vec![0.0; vector_size],
            density: vec![0.0; vector_size],
            initial_density: vec![0.0; vector_size],
            size,
        }
    }
}
