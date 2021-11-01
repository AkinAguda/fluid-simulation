use std::cmp;

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

// TYPES

// This is the type of a fluid property
pub type PropertyType = Vec<f32>;

// STRUCTS
pub fn lerp(a: f32, b: f32, k: f32) -> f32 {
    a + (k * (b - a))
}

pub fn pure_ix_fn(x: u16, y: u16, n: u16) -> usize {
    let mut new_x = cmp::min(x, n + 1);
    new_x = cmp::max(0, new_x);
    let mut new_y = cmp::min(y, n + 1);
    new_y = cmp::max(0, new_y);
    (new_x + (n + 2) * new_y) as usize
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
    ($n:expr, $b:expr, $property:expr) => {
        for i in 1..($n + 1) {
            $property[pure_ix_fn(0, i, $n)] = if ($b == 1) {
                -1.0 * $property[pure_ix_fn(1, i, $n)]
            } else {
                $property[pure_ix_fn(1, i, $n)]
            };
            $property[pure_ix_fn($n + 1, i, $n)] = if ($b == 1) {
                -1.0 * $property[pure_ix_fn($n, i, $n)]
            } else {
                $property[pure_ix_fn($n, i, $n)]
            };
            $property[pure_ix_fn(i, 0, $n)] = if ($b == 2) {
                -1.0 * $property[pure_ix_fn(i, 1, $n)]
            } else {
                $property[pure_ix_fn(i, 1, $n)]
            };
            $property[pure_ix_fn(i, $n + 1, $n)] = if ($b == 2) {
                -1.0 * $property[pure_ix_fn(i, $n, $n)]
            } else {
                $property[pure_ix_fn(i, $n, $n)]
            };
        }
        $property[pure_ix_fn(0, 0, $n)] =
            0.5 * ($property[pure_ix_fn(1, 0, $n)] + $property[pure_ix_fn(0, 1, $n)]);
        $property[pure_ix_fn(0, $n + 1, $n)] =
            0.5 * ($property[pure_ix_fn(1, $n + 1, $n)] + $property[pure_ix_fn(0, $n + 1, $n)]);
        $property[pure_ix_fn($n + 1, 0, $n)] =
            0.5 * ($property[pure_ix_fn($n + 1, 0, $n)] + $property[pure_ix_fn($n + 1, 1, $n)]);
        $property[pure_ix_fn($n + 1, $n + 1, $n)] =
            0.5 * ($property[pure_ix_fn($n, $n + 1, $n)] + $property[pure_ix_fn($n + 1, $n, $n)]);
    };
}

#[macro_export]
macro_rules! advect {
    ($n:expr, $b:expr, $property:expr, $prev_property:expr, $velocity_x:expr, $velocity_y:expr, $dt:expr) => {
        for i in 1..$n + 1 {
            for j in 1..$n + 1 {
                let index = pure_ix_fn(i, j, $n) as usize;

                let inital_pos_x = i as f32 - $velocity_x[pure_ix_fn(i, j, $n)] * $dt;
                let inital_pos_y = i as f32 - $velocity_y[pure_ix_fn(i, j, $n)] * $dt;

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
                        $prev_property[pure_ix_fn(point_1_x, point_1_y, $n)],
                        $prev_property[pure_ix_fn(point_2_x, point_2_y, $n)],
                        imaginary_x,
                    ),
                    lerp(
                        $prev_property[pure_ix_fn(point_3_x, point_3_y, $n)],
                        $prev_property[pure_ix_fn(point_4_x, point_4_y, $n)],
                        imaginary_x,
                    ),
                    imaginary_y,
                );
            }
        }

        set_bnd!($n, $b, $property);
    };
}

#[macro_export]
macro_rules! project {
    ($n:expr, $velocity_x:expr, $velocity_y:expr, $poisson:expr, $divergence_values:expr) => {
        for i in 1..$n + 1 {
            for j in 1..$n + 1 {
                let index = pure_ix_fn(i, j, $n);
                let a =
                    $velocity_x[pure_ix_fn(i + 1, j, $n)] - $velocity_x[pure_ix_fn(i - 1, j, $n)];
                let b =
                    $velocity_y[pure_ix_fn(i, j + 1, $n)] - $velocity_y[pure_ix_fn(i, j + 1, $n)];

                $divergence_values[index] = 0.5 * (a + b);
                $poisson[index] = 0.0;
            }
        }

        set_bnd!($n, 0, $divergence_values);
        set_bnd!($n, 0, $poisson);

        for _ in 0..10 {
            for i in 1..$n + 1 {
                for j in 1..$n + 1 {
                    let index = pure_ix_fn(i, j, $n);
                    $poisson[index] = ($poisson[pure_ix_fn(i - 1, j, $n)]
                        + $poisson[pure_ix_fn(i + 1, j, $n)]
                        + $poisson[pure_ix_fn(i, j - 1, $n)]
                        + $poisson[pure_ix_fn(i, j + 1, $n)]
                        - $divergence_values[index])
                        / 4.0
                }
            }
        }

        set_bnd!($n, 0, $poisson);

        for i in 1..$n + 1 {
            for j in 1..$n + 1 {
                let index = pure_ix_fn(i, j, $n);
                $velocity_x[index] -=
                    ($poisson[pure_ix_fn(i + 1, j, $n)] - $poisson[pure_ix_fn(i - 1, j, $n)]) * 0.5;
                $velocity_y[index] -=
                    ($poisson[pure_ix_fn(i, j + 1, $n)] - $poisson[pure_ix_fn(i, j - 1, $n)]) * 0.5;
            }
        }
        set_bnd!($n, 1, $velocity_x);
        set_bnd!($n, 2, $velocity_y);
    };
}

#[macro_export]
macro_rules! diffuse {
    ($n:expr, $b:expr, $property:expr, $prev_property:expr, $diffusion:expr, $dt:expr) => {
        let k = $dt * $diffusion * $n as f32 * $n as f32;
        for _ in 0..10 {
            for i in 1..$n + 1 {
                for j in 1..$n + 1 {
                    let index = pure_ix_fn(i, j, $n) as usize;

                    $property[index] = ($prev_property[index]
                        + (k * ($property[pure_ix_fn(i - 1, j, $n) as usize]
                            + $property[pure_ix_fn(i + 1, j, $n) as usize]
                            + $property[pure_ix_fn(i, j - 1, $n) as usize]
                            + $property[pure_ix_fn(i, j + 1, $n) as usize]))
                            / 4.0)
                        / (1.0 + k)
                }
            }

            set_bnd!($n, $b, $property);
        }
    };
}
