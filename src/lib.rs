mod utils;

use utils::{
    gauss_seidel, interpolate, val_after_diff, DiffLinearEquationArgs, GaussSeidelFunction,
    PropertyType,
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
    velocity_x: PropertyType,
    velocity_y: PropertyType,
    initial_velocity_x: PropertyType,
    initial_velocity_y: PropertyType,
    density: PropertyType,
    initial_density: PropertyType,
    size: u16,
}

#[wasm_bindgen]
impl Fluid {
    pub fn new(config: FluidConfig) -> Fluid {
        let size = (config.n + 2) * (config.n + 2);
        let vector_size = size.into();
        Fluid {
            config,
            dt: 0.1,
            velocity_x: vec![0.0; vector_size],
            velocity_y: vec![0.0; vector_size],
            initial_velocity_x: vec![0.0; vector_size],
            initial_velocity_y: vec![0.0; vector_size],
            density: vec![0.0; vector_size],
            initial_density: vec![0.0; vector_size],
            size,
        }
    }
    pub fn ix(&self, x: u16, y: u16) -> u16 {
        x + (self.config.n + 2) * y
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

    fn diffuse_density(&mut self) {
        for i in 1..self.config.n + 1 {
            for j in 1..self.config.n + 1 {
                let index = self.ix(i, j) as usize;
                self.density[index] = self.diffuse(i, j, &self.initial_density);
            }
        }
    }

    fn diffuse_velocity(&mut self) {
        for i in 1..self.config.n + 1 {
            for j in 1..self.config.n + 1 {
                let index = self.ix(i, j) as usize;
                self.velocity_x[index] = self.diffuse(i, j, &self.initial_velocity_x);
                self.velocity_y[index] = self.diffuse(i, j, &self.initial_velocity_y);
            }
        }
    }

    fn advect(&self, x: u16, y: u16, property: &PropertyType) -> f64 {
        // This calculates the position for where the previous density would come from
        let initial_pos_x = x as f64 - self.velocity_x[self.ix(x, y) as usize] * self.dt;
        let initial_pos_y = y as f64 - self.velocity_y[self.ix(x, y) as usize] * self.dt;

        let point_1 = [initial_pos_x.floor(), initial_pos_y.floor()]; // top left
        let point_2 = [initial_pos_x.ceil(), initial_pos_y.floor()]; // top right
        let point_3 = [initial_pos_x.floor(), initial_pos_y.ceil()]; //  bottom left
        let point_4 = [initial_pos_x.ceil(), initial_pos_y.ceil()]; //  bottom right

        // This does some bilinear interpolation
        let linear_interpolation_of_top = interpolate(
            point_1[0],
            property[self.ix(point_1[0] as u16, point_1[1] as u16) as usize],
            point_2[0],
            property[self.ix(point_2[0] as u16, point_2[1] as u16) as usize],
            initial_pos_x,
        );

        let linear_interpolation_of_bottom = interpolate(
            point_3[0],
            property[self.ix(point_3[0] as u16, point_3[1] as u16) as usize],
            point_4[0],
            property[self.ix(point_4[0] as u16, point_4[1] as u16) as usize],
            initial_pos_x,
        );

        interpolate(
            point_1[1],
            linear_interpolation_of_top,
            point_3[1],
            linear_interpolation_of_bottom,
            initial_pos_y,
        )
    }

    fn advect_density(&mut self) {
        for i in 1..self.config.n + 1 {
            for j in 1..self.config.n + 1 {
                let index = self.ix(i, j) as usize;
                self.density[index] = self.advect(i, j, &self.initial_density);
            }
        }
    }

    fn advect_velocity(&mut self) {
        for i in 1..self.config.n + 1 {
            for j in 1..self.config.n + 1 {
                let index = self.ix(i, j) as usize;
                self.velocity_x[index] = self.advect(i, j, &self.initial_velocity_x);
                self.velocity_y[index] = self.advect(i, j, &self.initial_velocity_y);
            }
        }
    }

    fn density_step(&mut self) {
        self.diffuse_density();
        std::mem::swap(&mut self.density, &mut self.initial_density);
        self.advect_density();
        std::mem::swap(&mut self.density, &mut self.initial_density);
    }

    fn velocity_step(&mut self) {
        self.diffuse_velocity();
        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);
        self.advect_velocity();
        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);
    }

    pub fn add_density(&mut self, index: usize, value: f64) {
        self.initial_density[index] += self.dt * value
    }

    pub fn add_velocity(&mut self, index: usize, value_x: f64, value_y: f64) {
        self.initial_velocity_x[index] += self.dt * value_x;
        self.initial_velocity_y[index] += self.dt * value_y;
    }

    pub fn simulate(&mut self) {
        self.velocity_step();
        self.density_step();
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
