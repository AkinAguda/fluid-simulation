mod utils;
use std::cmp;

use utils::{
    lerp, set_panic_hook, val_after_diff, val_after_poisson, DiffLinearEquationArgs,
    GaussSeidelFunction, PropertyType,
};
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
    fn log_f64(a: f64, b: &str);

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
    divergence_values: PropertyType,
    poisson_values: PropertyType,
    size: u16,
}

#[wasm_bindgen]
impl Fluid {
    pub fn new(config: FluidConfig) -> Fluid {
        set_panic_hook();
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
            divergence_values: vec![0.0; vector_size],
            poisson_values: vec![0.0; vector_size],
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

    fn diffuse_density(&mut self) {
        let k = self.dt * self.config.diffusion * (self.config.n as f64) * (self.config.n as f64);
        for _ in 0..10 {
            for i in 1..self.config.n + 1 {
                for j in 1..self.config.n + 1 {
                    let index = self.ix(i, j) as usize;
                    // self.density[index] = self.diffuse(i, j, &self.initial_density);
                    self.density[index] = (self.initial_density[self.ix(i, j) as usize]
                        + k * (self.density[self.ix(i - 1, j) as usize]
                            + self.density[self.ix(i + 1, j) as usize]
                            + self.density[self.ix(i, j - 1) as usize]
                            + self.density[self.ix(i, j + 1) as usize]))
                        / (1.0 + 4.0 * k)
                }
            }
        }
    }

    fn diffuse_velocity(&mut self) {
        let k = self.dt * self.config.diffusion * (self.config.n as f64) * (self.config.n as f64);
        for _ in 0..10 {
            for i in 1..self.config.n + 1 {
                for j in 1..self.config.n + 1 {
                    let index = self.ix(i, j) as usize;
                    // self.density[index] = self.diffuse(i, j, &self.initial_density);
                    self.velocity_x[index] = (self.initial_velocity_x[self.ix(i, j) as usize]
                        + k * (self.density[self.ix(i - 1, j) as usize]
                            + self.velocity_x[self.ix(i + 1, j) as usize]
                            + self.velocity_x[self.ix(i, j - 1) as usize]
                            + self.velocity_x[self.ix(i, j + 1) as usize])
                            / 4.0)
                        / (1.0 + k)
                }
            }
        }
        // std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);

        for _ in 0..10 {
            for i in 1..self.config.n + 1 {
                for j in 1..self.config.n + 1 {
                    let index = self.ix(i, j) as usize;
                    // self.density[index] = self.diffuse(i, j, &self.initial_density);
                    self.velocity_y[index] = (self.initial_velocity_y[self.ix(i, j) as usize]
                        + k * (self.density[self.ix(i - 1, j) as usize]
                            + self.velocity_y[self.ix(i + 1, j) as usize]
                            + self.velocity_y[self.ix(i, j - 1) as usize]
                            + self.velocity_y[self.ix(i, j + 1) as usize])
                            / 4.0)
                        / (1.0 + k)
                }
            }
        }
    }

    fn advect(&self, x: u16, y: u16, property: &PropertyType) -> f64 {
        // This calculates the position for where the previous density would come from
        let initial_pos_x = x as f64 - self.velocity_x[self.ix(x, y) as usize] * self.dt;
        let initial_pos_y = y as f64 - self.velocity_y[self.ix(x, y) as usize] * self.dt;

        let i_x = initial_pos_x.floor();
        let i_y = initial_pos_y.floor();

        let j_x = initial_pos_x.fract();
        let j_y = initial_pos_y.fract();

        let z1 = lerp(
            property[self.ix(i_x as u16, i_y as u16) as usize],
            property[self.ix(i_x as u16 + 1, i_y as u16) as usize],
            j_x,
        );
        let z2 = lerp(
            property[self.ix(i_x as u16, i_y as u16 + 1) as usize],
            property[self.ix(i_x as u16 + 1, i_y as u16 + 1) as usize],
            j_x,
        );

        lerp(z1, z2, j_y)
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

    fn divergence(&mut self, x: u16, y: u16) -> f64 {
        (self.velocity_x[self.ix(x + 1, y) as usize] - self.velocity_x[self.ix(x - 1, y) as usize]
            + self.velocity_y[self.ix(x, y + 1) as usize]
            - self.velocity_y[self.ix(x, y - 1) as usize])
            / 2.0
    }

    fn project(&mut self) {
        for i in 1..self.config.n + 1 {
            for j in 1..self.config.n + 1 {
                let index = self.ix(i, j) as usize;
                self.divergence_values[index] = self.divergence(i, j);
            }
        }

        for _ in 0..10 {
            for i in 1..self.config.n + 1 {
                for j in 1..self.config.n + 1 {
                    let index = self.ix(i, j) as usize;

                    self.poisson_values[index] = val_after_poisson(
                        &vec![
                            self.poisson_values[self.ix(i - 1, j) as usize],
                            self.poisson_values[self.ix(i + 1, j) as usize],
                            self.poisson_values[self.ix(i, j - 1) as usize],
                            self.poisson_values[self.ix(i, j + 1) as usize],
                        ],
                        &self.divergence_values[index],
                    );
                }
            }
        }

        for _ in 0..10 {
            for i in 1..self.config.n + 1 {
                for j in 1..self.config.n + 1 {
                    let index = self.ix(i, j) as usize;
                    self.velocity_x[index] = self.velocity_x[index]
                        - ((self.poisson_values[self.ix(i + 1, j) as usize])
                            - (self.poisson_values[self.ix(i - 1, j) as usize]))
                            / 2.0;
                    // log_f64(self.velocity_x[index], "velx after project");
                    self.velocity_y[index] = self.velocity_y[index]
                        - ((self.poisson_values[self.ix(i, j + 1) as usize])
                            - (self.poisson_values[self.ix(i, j - 1) as usize]))
                            / 2.0;

                    // log_f64(self.velocity_x[index], "vely after project");
                }
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
        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);
        self.diffuse_velocity();
        self.project();
        std::mem::swap(&mut self.velocity_x, &mut self.initial_velocity_x);
        std::mem::swap(&mut self.velocity_y, &mut self.initial_velocity_y);
        self.advect_velocity();
        self.project();
    }

    pub fn add_density(&mut self, index: usize, value: f64) {
        self.initial_density[index] += self.dt * value
    }

    pub fn add_velocity(&mut self, index: usize, value_x: f64, value_y: f64) {
        self.initial_velocity_x[index] = value_x;
        self.initial_velocity_y[index] = value_y;
        self.velocity_x[index] = self.dt * value_x;
        self.velocity_y[index] = self.dt * value_y;
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

    pub fn get_velocity_X(&self, index: usize) -> f64 {
        self.velocity_x[index]
    }
    pub fn get_velocity_y(&self, index: usize) -> f64 {
        self.velocity_y[index]
    }
}
