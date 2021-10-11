mod utils;

use utils::{
    gauss_seidel, val_after_diff, DiffLinearEquationArgs, GaussSeidelFunction, PropertyType,
};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct FluidConfig {
    n: u16,
    diffusion: f64,
}

#[wasm_bindgen]
impl FluidConfig {
    pub fn new(n: u16, diffusion: f64) -> FluidConfig {
        FluidConfig { n, diffusion }
    }
}

#[wasm_bindgen]
pub struct Fluid {
    config: FluidConfig,
    dt: f64,
    // velocity_x: PropertyType,
    // velocityY: PropertyType,
    // initial_velocity_x: PropertyType,
    // initial_velocityY: PropertyType,
    density: PropertyType,
    initial_density: PropertyType,
    size: u16,
}

#[wasm_bindgen]
impl Fluid {
    pub fn new(config: FluidConfig, dt: f64) -> Fluid {
        let size = (config.n + 2) * (config.n + 2);
        let vector_size = size.into();
        Fluid {
            config,
            dt,
            // velocity_x: vec![0.0; vector_size],
            // velocityY: vec![0.0; vector_size],
            // initial_velocity_x: vec![0.0; vector_size],
            // initial_velocityY: vec![0.0; vector_size],
            density: vec![0.0; vector_size],
            initial_density: vec![0.0; vector_size],
            size,
        }
    }
    fn ix(&self, x: u16, y: u16) -> u16 {
        x + (self.config.n + 2) * y
    }

    fn add_source(&self, property: &mut PropertyType, initial_property: &PropertyType) {
        for index in 0..self.size as usize {
            property[index] += self.dt * initial_property[index]
        }
    }

    fn diffuse(&self, x: u16, y: u16, property: &PropertyType) -> f64 {
        let k = self.dt * self.config.diffusion;

        let gauss_seidel_fn1 = GaussSeidelFunction::new(
            val_after_diff,
            DiffLinearEquationArgs::new(property[self.ix(x + 1, y) as usize], k),
        );

        let gauss_seidel_fn2 = GaussSeidelFunction::new(
            val_after_diff,
            DiffLinearEquationArgs::new(property[self.ix(x - 1, y) as usize], k),
        );

        let gauss_seidel_fn3 = GaussSeidelFunction::new(
            val_after_diff,
            DiffLinearEquationArgs::new(property[self.ix(x, y + 1) as usize], k),
        );

        let gauss_seidel_fn4 = GaussSeidelFunction::new(
            val_after_diff,
            DiffLinearEquationArgs::new(property[self.ix(x, y - 1) as usize], k),
        );

        let surrounding_values = gauss_seidel(
            vec![
                gauss_seidel_fn1,
                gauss_seidel_fn2,
                gauss_seidel_fn3,
                gauss_seidel_fn4,
            ],
            vec![0.0, 0.0, 0.0, 0.0],
            10,
        );

        val_after_diff(
            &surrounding_values,
            &DiffLinearEquationArgs::new(property[self.ix(x, y) as usize], k),
        )
    }

    fn diffusion_step(&self, property: &mut PropertyType, initial_property: &PropertyType) {
        for i in 1..self.config.n + 1 {
            for j in 1..self.config.n + 1 {
                let index = self.ix(i, j) as usize;
                property[index] = self.diffuse(i, j, initial_property);
            }
        }
    }

    fn density_step(&mut self) {
        // add_source!(
        //     self.density,
        //     &self.initial_density,
        //     self.size as usize,
        //     self.dt
        // );
        // self.diffusion_step(&self.density, &self.initial_density);
    }

    pub fn get_density_at_index(&self, index: usize) -> f64 {
        self.density[index]
    }

    pub fn get_n(&self) -> u16 {
        self.config.n
    }

    pub fn get_size(&self) -> u16 {
        self.size
    }

    pub fn set_dt(&mut self, dt: f64) {
        self.dt = dt
    }
}
