mod utils;

use utils::{gauss_seidel, val_after_diff, DiffLinearEquationArgs, LinearEquation, PropertyType};
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
    fn add_source(&self, property: &mut PropertyType, initial_property: &PropertyType) {
        for index in 0..self.size as usize {
            property[index] += self.config.dt * initial_property[index]
        }
    }

    // fn val_after_diff(&self, x: u16, y: u16, property: &PropertyType) -> LinearEquation<f64> {
    //     |surrounding_property_values: PropertyType| -> f64 {
    //         let k = self.config.dt * self.config.diffusion;
    //         (property[self.ix(x, y) as usize]
    //             + (k * (surrounding_property_values[0]
    //                 + surrounding_property_values[1]
    //                 + surrounding_property_values[2]
    //                 + surrounding_property_values[3]))
    //                 / 4.0)
    //             / (1.0 + k)
    //     }
    // }
    fn diffuse(&self, x: u16, y: u16, property: &PropertyType) -> f64 {
        let k = self.config.dt * self.config.diffusion;
        // val_after_diff(x, y, property)(&gauss_seidel(
        //     vec![a, a, a, a],
        //     vec![0.0, 0.0, 0.0, 0.0],
        //     10,
        //     vec![
        //         DiffLinearEquationArgs::new(property[self.ix(x + 1, y) as usize], k),
        //         DiffLinearEquationArgs::new(property[self.ix(x - 1, y) as usize], k),
        //         DiffLinearEquationArgs::new(property[self.ix(x, y + 1) as usize], k),
        //         DiffLinearEquationArgs::new(property[self.ix(x, y - 1) as usize], k),
        //     ],
        // ))
        val_after_diff(
            gauss_seidel(
                vec![
                    val_after_diff,
                    val_after_diff,
                    val_after_diff,
                    val_after_diff,
                ],
                vec![0.0, 0.0, 0.0, 0.0],
                10,
                vec![
                    Some(DiffLinearEquationArgs::new(
                        property[self.ix(x + 1, y) as usize],
                        k,
                    )),
                    Some(DiffLinearEquationArgs::new(
                        property[self.ix(x - 1, y) as usize],
                        k,
                    )),
                    Some(DiffLinearEquationArgs::new(
                        property[self.ix(x, y + 1) as usize],
                        k,
                    )),
                    Some(DiffLinearEquationArgs::new(
                        property[self.ix(x, y - 1) as usize],
                        k,
                    )),
                ],
            ),
            DiffLinearEquationArgs::new(property[self.ix(x, y) as usize], k),
        )
    }
}
