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
pub type PropertyType = Vec<f64>;

// STRUCTS
pub fn lerp(a: f64, b: f64, k: f64) -> f64 {
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
    ($property:expr, $initial_property:expr, $size:expr, $dt:expr) => {
        for index in 0..$size {
            $property[index] += $dt * $initial_property[index];
            $initial_property[index] = 0.0;
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
        let dt0 = $dt * ($n as f64);

        for i in 1..$n + 1 {
            for j in 1..$n + 1 {
                let index = pure_ix_fn(i, j, $n) as usize;
                let initial_pos_x = i as f64 - $velocity_x[index] * dt0;
                let initial_pos_y = j as f64 - $velocity_y[index] * dt0;

                // log_f64($velocity_x[index], "vel x");
                // log_f64(j as f64, "y");
                // log_f64(initial_pos_x, "new x");
                // log_f64(initial_pos_y, "new y");

                let initial_pos_x = if initial_pos_x < 0.5 {
                    0.5
                } else {
                    initial_pos_x
                };

                let initial_pos_x = if initial_pos_x > $n as f64 + 0.5 {
                    0.5 + $n as f64
                } else {
                    initial_pos_x
                };

                let initial_pos_y = if initial_pos_y < 0.5 {
                    0.5
                } else {
                    initial_pos_y
                };

                let initial_pos_y = if initial_pos_y > $n as f64 + 0.5 {
                    0.5 + $n as f64
                } else {
                    initial_pos_y
                };

                let i_x = initial_pos_x.floor();
                let i_y = initial_pos_y.floor();

                let j_x = initial_pos_x.fract();
                let j_y = initial_pos_y.fract();

                let z1 = lerp(
                    $prev_property[pure_ix_fn(i_x as u16, i_y as u16, $n) as usize],
                    $prev_property[pure_ix_fn(i_x as u16 + 1, i_y as u16, $n) as usize],
                    j_x,
                );
                let z2 = lerp(
                    $prev_property[pure_ix_fn(i_x as u16, i_y as u16 + 1, $n) as usize],
                    $prev_property[pure_ix_fn(i_x as u16 + 1, i_y as u16 + 1, $n) as usize],
                    j_x,
                );

                $property[index] = lerp(z1, z2, j_y);
            }
        }

        set_bnd!($n, $b, $property);
    };
}

#[macro_export]
macro_rules! project {
    ($n:expr, $velocity_x:expr, $velocity_y:expr, $poisson:expr, $divergence_values:expr) => {
        let h = 1 / $n;
        for i in 1..$n + 1 {
            for j in 1..$n + 1 {
                let index = pure_ix_fn(i, j, $n);
                // $divergence_values[index] = ($velocity_x[pure_ix_fn(i + 1, j, $n)]
                //     - $velocity_x[pure_ix_fn(i - 1, j, $n)]
                //     + $velocity_y[pure_ix_fn(i, j + 1, $n)]
                //     - $velocity_y[pure_ix_fn(i, j - 1, $n)])
                //     / 2.0
                $divergence_values[index] = -0.5
                    * h as f64
                    * ($velocity_x[pure_ix_fn(i + 1, j, $n)]
                        - $velocity_x[pure_ix_fn(i - 1, j, $n)]
                        + $velocity_y[pure_ix_fn(i, j + 1, $n)]
                        - $velocity_y[pure_ix_fn(i, j + 1, $n)]);
                $poisson[index] = 0.0;
            }
        }

        set_bnd!($n, 0, $divergence_values);
        set_bnd!($n, 0, $poisson);

        // ($poisson[pure_ix_fn(i - 1, j, $n)]
        // + $poisson[pure_ix_fn(i + 1, j, $n)]
        // + $poisson[pure_ix_fn(i, j - 1, $n)]
        // + $poisson[pure_ix_fn(i, j + 1, $n)]
        // + $divergence_values[index])
        // / 4.0

        for _ in 0..10 {
            for i in 1..$n + 1 {
                for j in 1..$n + 1 {
                    let index = pure_ix_fn(i, j, $n);
                    $poisson[index] = ($divergence_values[index]
                        + $poisson[pure_ix_fn(i - 1, j, $n)]
                        + $poisson[pure_ix_fn(i + 1, j, $n)]
                        + $poisson[pure_ix_fn(i, j - 1, $n)]
                        + $poisson[pure_ix_fn(i, j + 1, $n)])
                        / 4.0
                }
            }
        }

        set_bnd!($n, 0, $poisson);

        for _ in 0..10 {
            for i in 1..$n + 1 {
                for j in 1..$n + 1 {
                    let index = pure_ix_fn(i, j, $n);
                    $velocity_x[index] = $velocity_x[index]
                        - (($poisson[pure_ix_fn(i + 1, j, $n)])
                            - ($poisson[pure_ix_fn(i - 1, j, $n)]))
                            / 2.0;
                    $velocity_y[index] = $velocity_y[index]
                        - (($poisson[pure_ix_fn(i, j + 1, $n)])
                            - ($poisson[pure_ix_fn(i, j - 1, $n)]))
                            / 2.0;
                }
            }
        }
        set_bnd!($n, 1, $velocity_x);
        set_bnd!($n, 2, $velocity_y);
    };
}

#[macro_export]
macro_rules! diffuse {
    ($n:expr, $b:expr, $property:expr, $prev_property:expr, $diffusion:expr, $dt:expr) => {
        let k = $dt * $diffusion * $n as f64 * $n as f64;
        for _ in 0..10 {
            for i in 1..$n + 1 {
                for j in 1..$n + 1 {
                    let index = pure_ix_fn(i, j, $n) as usize;

                    $property[index] = ($prev_property[index]
                        + k * ($property[pure_ix_fn(i - 1, j, $n) as usize]
                            + $property[pure_ix_fn(i + 1, j, $n) as usize]
                            + $property[pure_ix_fn(i, j - 1, $n) as usize]
                            + $property[pure_ix_fn(i, j + 1, $n) as usize]))
                        / (1.0 + 4.0 * k)
                }
            }

            set_bnd!($n, $b, $property);
        }
    };
}
