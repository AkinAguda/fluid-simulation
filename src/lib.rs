mod utils;
use std::cmp;

use utils::{lerp, pure_ix_fn, set_panic_hook, PropertyType};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f32(a: f32, b: &str);

    // log usize and what it represents
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_usize(a: usize, b: &str);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub struct FluidConfig {
    n: u16,
    diffusion: f32,
}

#[wasm_bindgen]
impl FluidConfig {
    pub fn new(n: u16, diffusion: f32) -> FluidConfig {
        FluidConfig { n, diffusion }
    }
}

#[wasm_bindgen]
pub struct Fluid {
    config: FluidConfig,
    dt: f32,
    velocity_x: PropertyType,
    velocity_y: PropertyType,
    initial_velocity_x: PropertyType,
    initial_velocity_y: PropertyType,
    velocity_x_source: PropertyType,
    velocity_y_source: PropertyType,
    density: PropertyType,
    initial_density: PropertyType,
    density_source: PropertyType,
    poisson_values: PropertyType,
    divergence_values: PropertyType,
    size: u16,
}

#[wasm_bindgen]
impl Fluid {
    pub fn new(config: FluidConfig, dt: f32) -> Fluid {
        set_panic_hook();
        let size = (config.n + 2) * (config.n + 2);
        let vector_size = size.into();
        Fluid {
            config,
            dt,
            velocity_x: vec![0.0; vector_size],
            velocity_y: vec![0.0; vector_size],
            initial_velocity_x: vec![0.0; vector_size],
            initial_velocity_y: vec![0.0; vector_size],
            velocity_x_source: vec![0.0; vector_size],
            velocity_y_source: vec![0.0; vector_size],
            density: vec![0.0; vector_size],
            initial_density: vec![0.0; vector_size],
            density_source: vec![0.0; vector_size],
            poisson_values: vec![0.0; vector_size],
            divergence_values: vec![0.0; vector_size],
            size,
        }
    }
    pub fn ix(&self, x: u16, y: u16) -> u16 {
        let mut new_x = cmp::min(x, self.config.n + 1);
        new_x = cmp::max(0, new_x);
        let mut new_y = cmp::min(y, self.config.n + 1);
        new_y = cmp::max(0, new_y);
        new_x + (self.config.n + 2) * new_y
    }

    fn density_step(&mut self) {
        add_source!(
            self.initial_density,
            self.density_source,
            self.size as usize,
            self.dt
        );

        diffuse!(
            self.config.n,
            0,
            self.density,
            self.initial_density,
            self.config.diffusion,
            self.dt
        );

        std::mem::swap(&mut self.density, &mut self.initial_density);

        advect!(
            self.config.n,
            0,
            self.density,
            self.initial_density,
            self.velocity_x,
            self.velocity_y,
            self.dt
        );

        std::mem::swap(&mut self.density, &mut self.initial_density);
    }

    fn velocity_step(&mut self) {
        add_source!(
            self.velocity_x,
            self.velocity_x_source,
            self.size as usize,
            self.dt
        );

        add_source!(
            self.velocity_y,
            self.velocity_y_source,
            self.size as usize,
            self.dt
        );

        diffuse!(
            self.config.n,
            1,
            self.velocity_x,
            self.initial_velocity_x,
            self.config.diffusion,
            self.dt
        );

        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);

        diffuse!(
            self.config.n,
            2,
            self.velocity_y,
            self.initial_velocity_y,
            self.config.diffusion,
            self.dt
        );

        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);

        project!(
            self.config.n,
            self.velocity_x,
            self.velocity_y,
            self.poisson_values,
            self.divergence_values
        );

        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);

        advect!(
            self.config.n,
            1,
            self.velocity_x,
            self.initial_velocity_x,
            self.velocity_x,
            self.velocity_y,
            self.dt
        );

        advect!(
            self.config.n,
            2,
            self.velocity_y,
            self.initial_velocity_y,
            self.velocity_x,
            self.velocity_y,
            self.dt
        );

        project!(
            self.config.n,
            self.velocity_x,
            self.velocity_y,
            self.poisson_values,
            self.divergence_values
        );

        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);
    }

    pub fn add_density(&mut self, index: usize, value: f32) {
        self.density_source[index] = value;
    }

    pub fn add_velocity(&mut self, index: usize, value_x: f32, value_y: f32) {
        self.velocity_x_source[index] = value_x;
        self.velocity_y_source[index] = value_y;
    }

    pub fn simulate(&mut self) {
        self.velocity_step();
        self.density_step();
    }

    pub fn get_density_at_index(&self, index: usize) -> f32 {
        self.density[index]
    }

    pub fn get_n(&self) -> u16 {
        self.config.n
    }

    pub fn get_size(&self) -> u16 {
        self.size
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = dt
    }

    pub fn get_velocity_X(&self, index: usize) -> f32 {
        self.velocity_x[index]
    }
    pub fn get_velocity_y(&self, index: usize) -> f32 {
        self.velocity_y[index]
    }
    pub fn get_density_expensive(&self) -> PropertyType {
        self.density.clone()
    }

    pub fn get_velocity_x_expensive(&self) -> PropertyType {
        self.velocity_x.clone()
    }

    pub fn get_velocity_y_expensive(&self) -> PropertyType {
        self.velocity_y.clone()
    }
}
