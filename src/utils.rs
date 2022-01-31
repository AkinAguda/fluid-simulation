use std::cmp;
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// ENUMS
pub enum BoundaryType {
    VERTICAL,
    HORIZONTAL,
    NONE,
}

// TYPES

// This is the type of a fluid property
pub type PropertyType = Vec<f32>;

// STRUCTS
pub fn lerp(a: f32, b: f32, k: f32) -> f32 {
    a + (k * (b - a))
}

#[wasm_bindgen]
pub fn pure_ix_fn(x: u16, y: u16, nw: u16, nh: u16) -> usize {
    let mut new_x = cmp::min(x, nw + 1);
    new_x = cmp::max(0, new_x);
    let mut new_y = cmp::min(y, nh + 1);
    new_y = cmp::max(0, new_y);
    ((new_x + new_y) + ((nw + 1) * new_y)) as usize
}

#[macro_export]
macro_rules! add_source {
    ($property:expr, $source:expr, $size:expr, $dt:expr) => {
        for index in 0..$size {
            $property[index] += $dt * $source[index];
            $source[index] = 0.0;
        }
    };
}

#[macro_export]
macro_rules! set_bnd {
    ($nw:expr, $nh:expr, $b:expr, $property:expr) => {
        let max = cmp::max($nw, $nh);
        for i in 1..(max + 1) {
            $property[pure_ix_fn(0, i, $nw, $nh)] = match $b {
                BoundaryType::VERTICAL => -$property[pure_ix_fn(1, i, $nw, $nh)],
                _ => $property[pure_ix_fn(1, i, $nw, $nh)],
            };

            $property[pure_ix_fn($nw + 1, i, $nw, $nh)] = match $b {
                BoundaryType::VERTICAL => -$property[pure_ix_fn($nw, i, $nw, $nh)],
                _ => $property[pure_ix_fn($nw, i, $nw, $nh)],
            };

            $property[pure_ix_fn(i, 0, $nw, $nh)] = match $b {
                BoundaryType::HORIZONTAL => -$property[pure_ix_fn(i, 1, $nw, $nh)],
                _ => $property[pure_ix_fn(i, 1, $nw, $nh)],
            };

            $property[pure_ix_fn(i, $nh + 1, $nw, $nh)] = match $b {
                BoundaryType::HORIZONTAL => -$property[pure_ix_fn(i, $nh, $nw, $nh)],
                _ => $property[pure_ix_fn(i, $nh, $nw, $nh)],
            };
        }

        $property[pure_ix_fn(0, 0, $nw, $nh)] =
            0.5 * ($property[pure_ix_fn(1, 0, $nw, $nh)] + $property[pure_ix_fn(0, 1, $nw, $nh)]);
        $property[pure_ix_fn(0, $nw + 1, $nw, $nh)] = 0.5
            * ($property[pure_ix_fn(1, $nw + 1, $nw, $nh)]
                + $property[pure_ix_fn(0, $nw + 1, $nw, $nh)]);
        $property[pure_ix_fn($nh + 1, 0, $nw, $nh)] = 0.5
            * ($property[pure_ix_fn($nh + 1, 0, $nw, $nh)]
                + $property[pure_ix_fn($nh + 1, 1, $nw, $nh)]);
        $property[pure_ix_fn($nh + 1, $nw + 1, $nw, $nh)] = 0.5
            * ($property[pure_ix_fn($nh, $nw + 1, $nw, $nh)]
                + $property[pure_ix_fn($nh + 1, $nw, $nw, $nh)]);
    };
}

#[macro_export]
macro_rules! advect {
    ($nw:expr, $nh:expr, $b:expr, $property:expr, $prev_property:expr, $velocity_x:expr, $velocity_y:expr, $dt:expr) => {
        for j in 1..$nh + 1 {
            for i in 1..$nw + 1 {
                let index = pure_ix_fn(i, j, $nw, $nh) as usize;

                let inital_pos_x = i as f32 - $velocity_x[pure_ix_fn(i, j, $nw, $nh)] * $dt;
                let inital_pos_y = j as f32 - $velocity_y[pure_ix_fn(i, j, $nw, $nh)] * $dt;

                let imaginary_x = inital_pos_x.fract();
                let imaginary_y = inital_pos_y.fract();

                let point_1_x = inital_pos_x.floor() as u16;
                let point_1_y = inital_pos_y.floor() as u16;

                let point_2_x = inital_pos_x.ceil() as u16;
                let point_2_y = inital_pos_y.floor() as u16;

                let point_3_x = inital_pos_x.floor() as u16;
                let point_3_y = inital_pos_y.ceil() as u16;

                let point_4_x = inital_pos_x.ceil() as u16;
                let point_4_y = inital_pos_y.ceil() as u16;

                $property[index] = lerp(
                    lerp(
                        $prev_property[pure_ix_fn(point_1_x, point_1_y, $nw, $nh)],
                        $prev_property[pure_ix_fn(point_2_x, point_2_y, $nw, $nh)],
                        imaginary_x,
                    ),
                    lerp(
                        $prev_property[pure_ix_fn(point_3_x, point_3_y, $nw, $nh)],
                        $prev_property[pure_ix_fn(point_4_x, point_4_y, $nw, $nh)],
                        imaginary_x,
                    ),
                    imaginary_y,
                );
            }
        }

        set_bnd!($nw, $nh, $b, $property);
    };
}

#[macro_export]
macro_rules! project {
    ($nw:expr, $nh:expr, $velocity_x:expr, $velocity_y:expr, $poisson_values:expr, $divergence_values:expr) => {
        for j in 1..$nh + 1 {
            for i in 1..$nw + 1 {
                let index = pure_ix_fn(i, j, $nw, $nh);
                let a = $velocity_x[pure_ix_fn(i + 1, j, $nw, $nh)]
                    - $velocity_x[pure_ix_fn(i - 1, j, $nw, $nh)];
                let b = $velocity_y[pure_ix_fn(i, j + 1, $nw, $nh)]
                    - $velocity_y[pure_ix_fn(i, j - 1, $nw, $nh)];

                $divergence_values[index] = 0.5 * (a + b);
                $poisson_values[index] = 0.0;
            }
        }

        set_bnd!($nw, $nh, BoundaryType::NONE, $divergence_values);
        set_bnd!($nw, $nh, BoundaryType::NONE, $poisson_values);

        for _ in 0..GAUSS_SEIDEL_ITERATIONS {
            for j in 1..$nh + 1 {
                for i in 1..$nw + 1 {
                    let index = pure_ix_fn(i, j, $nw, $nh);
                    $poisson_values[index] = ($poisson_values[pure_ix_fn(i - 1, j, $nw, $nh)]
                        + $poisson_values[pure_ix_fn(i + 1, j, $nw, $nh)]
                        + $poisson_values[pure_ix_fn(i, j - 1, $nw, $nh)]
                        + $poisson_values[pure_ix_fn(i, j + 1, $nw, $nh)]
                        - $divergence_values[index])
                        / 4.0
                }
            }
        }

        set_bnd!($nw, $nh, BoundaryType::NONE, $poisson_values);

        for j in 1..$nh + 1 {
            for i in 1..$nw + 1 {
                let index = pure_ix_fn(i, j, $nw, $nh);
                $velocity_x[index] -= ($poisson_values[pure_ix_fn(i + 1, j, $nw, $nh)]
                    - $poisson_values[pure_ix_fn(i - 1, j, $nw, $nh)])
                    * 0.5;
                $velocity_y[index] -= ($poisson_values[pure_ix_fn(i, j + 1, $nw, $nh)]
                    - $poisson_values[pure_ix_fn(i, j - 1, $nw, $nh)])
                    * 0.5;
            }
        }
        set_bnd!($nw, $nh, BoundaryType::VERTICAL, $velocity_x);
        set_bnd!($nw, $nh, BoundaryType::HORIZONTAL, $velocity_y);
    };
}

#[macro_export]
macro_rules! diffuse {
    ($nw:expr, $nh:expr, $b:expr, $property:expr, $prev_property:expr, $diffusion:expr, $dt:expr) => {
        let k = $dt * $diffusion;
        for _ in 0..GAUSS_SEIDEL_ITERATIONS {
            for j in 1..$nh + 1 {
                for i in 1..$nw + 1 {
                    let index = pure_ix_fn(i, j, $nw, $nh) as usize;

                    $property[index] = ($prev_property[index]
                        + (k * ($property[pure_ix_fn(i + 1, j, $nw, $nh) as usize]
                            + $property[pure_ix_fn(i - 1, j, $nw, $nh) as usize]
                            + $property[pure_ix_fn(i, j + 1, $nw, $nh) as usize]
                            + $property[pure_ix_fn(i, j - 1, $nw, $nh) as usize]))
                            / 4.0)
                        / (1.0 + k)
                }
            }

            set_bnd!($nw, $nh, $b, $property);
        }
    };
}
