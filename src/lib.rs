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
    dt: f64,
    diffusion: f64,
}

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
    pub fn new(config: FluidConfig) -> Fluid {
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
    fn ix(&self, x: u16, y: u16) -> u16 {
        x + (self.config.n + 2) * y
    }
    // fn add_source(&self, property: &mut PropertyType, initial_property: &PropertyType) {
    //     for index in 0..self.size as usize {
    //         property[index] += self.config.dt * initial_property[index]
    //     }
    // }

    fn diffuse(&self, x: u16, y: u16, property: &PropertyType) -> f64 {
        let k = self.config.dt * self.config.diffusion;

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
        let mut density = &self.density;

        // Add source
        for index in 0..self.size as usize {
            self.density[index] += self.config.dt * self.initial_density[index]
        }
    }

    // Eposed methods
    pub fn get_density_at_x_y(&self, x: u16, y: u16) -> f64 {
        self.density[self.ix(x, y) as usize]
    }
}
