mod constants;
mod utils;

use constants::GAUSS_SEIDEL_ITERATIONS;
use std::cmp;
use utils::{lerp, pure_ix_fn, set_panic_hook, BoundaryType, PropertyType};
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
    nw: u16,
    nh: u16,
    diffusion: f32,
}

#[wasm_bindgen]
impl FluidConfig {
    pub fn new(nw: u16, nh: u16, diffusion: f32) -> FluidConfig {
        FluidConfig { nw, nh, diffusion }
    }

    pub fn set_diffusion(&mut self, diffusion: f32) {
        self.diffusion = diffusion
    }

    pub fn get_diffusion(&self) -> f32 {
        self.diffusion
    }
}

#[wasm_bindgen]
pub struct Fluid {
    config: FluidConfig,
    dt: f32,
    empty_property: PropertyType,
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
        let size = (config.nw + 2) * (config.nh + 2);
        let vector_size = size.into();
        Fluid {
            config,
            dt,
            empty_property: vec![0.0; vector_size],
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

    fn density_step(&mut self) {
        add_source!(
            self.initial_density,
            self.density_source,
            self.size as usize,
            self.dt
        );

        diffuse!(
            self.config.nw,
            self.config.nh,
            BoundaryType::NONE,
            self.density,
            self.initial_density,
            self.config.diffusion,
            self.dt
        );

        std::mem::swap(&mut self.density, &mut self.initial_density);

        advect!(
            self.config.nw,
            self.config.nh,
            BoundaryType::NONE,
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
            self.initial_velocity_x,
            self.velocity_x_source,
            self.size as usize,
            self.dt
        );

        add_source!(
            self.initial_velocity_y,
            self.velocity_y_source,
            self.size as usize,
            self.dt
        );

        diffuse!(
            self.config.nw,
            self.config.nh,
            BoundaryType::VERTICAL,
            self.velocity_x,
            self.initial_velocity_x,
            self.config.diffusion,
            self.dt
        );

        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);

        diffuse!(
            self.config.nw,
            self.config.nh,
            BoundaryType::HORIZONTAL,
            self.velocity_y,
            self.initial_velocity_y,
            self.config.diffusion,
            self.dt
        );

        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);

        project!(
            self.config.nw,
            self.config.nh,
            self.velocity_x,
            self.velocity_y,
            self.poisson_values,
            self.divergence_values
        );

        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);

        advect!(
            self.config.nw,
            self.config.nh,
            BoundaryType::VERTICAL,
            self.velocity_x,
            self.initial_velocity_x,
            self.velocity_x,
            self.velocity_y,
            self.dt
        );

        advect!(
            self.config.nw,
            self.config.nh,
            BoundaryType::HORIZONTAL,
            self.velocity_y,
            self.initial_velocity_y,
            self.velocity_x,
            self.velocity_y,
            self.dt
        );

        project!(
            self.config.nw,
            self.config.nh,
            self.velocity_x,
            self.velocity_y,
            self.poisson_values,
            self.divergence_values
        );

        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);
    }

    // All public methods

    pub fn clear(&mut self) {
        self.velocity_x = self.empty_property.clone();
        self.velocity_y = self.empty_property.clone();
        self.initial_velocity_x = self.empty_property.clone();
        self.initial_velocity_y = self.empty_property.clone();
        self.velocity_x_source = self.empty_property.clone();
        self.velocity_y_source = self.empty_property.clone();
        self.density = self.empty_property.clone();
        self.initial_density = self.empty_property.clone();
        self.density_source = self.empty_property.clone();
        self.poisson_values = self.empty_property.clone();
        self.divergence_values = self.empty_property.clone();
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

    pub fn ix(&self, x: u16, y: u16) -> u16 {
        pure_ix_fn(x, y, self.config.nw, self.config.nh) as u16
    }

    pub fn get_nw(&self) -> u16 {
        self.config.nw
    }

    pub fn get_nh(&self) -> u16 {
        self.config.nh
    }

    pub fn get_size(&self) -> u16 {
        self.size
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = dt
    }

    pub fn get_velocity_x(&self, index: usize) -> f32 {
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

    pub fn set_config_diffusion(&mut self, value: f32) {
        self.config.set_diffusion(value)
    }
}
