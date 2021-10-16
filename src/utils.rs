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

// This specifies the type/pattern for a linear equation.
pub type LinearEquation<T> = fn(variables: &Vec<f64>, args: &T) -> f64;

// This is the type of a fluid property
pub type PropertyType = Vec<f64>;

// STRUCTS

/* This struct defines a specification for functions that can be passed
into the gauss_seidel function. The generic T defines the type of the argument
that will be passes inrto the linear equation for any internal computation that
might be needed */
pub struct GaussSeidelFunction<T> {
    pub funciton: LinearEquation<T>,
    pub args: T,
}

impl<T> GaussSeidelFunction<T> {
    pub fn new(funciton: LinearEquation<T>, args: T) -> GaussSeidelFunction<T> {
        GaussSeidelFunction { funciton, args }
    }
    pub fn call(&self, variables: &Vec<f64>) -> f64 {
        (self.funciton)(&variables, &self.args)
    }
}

/* This struct defines the parameters needed besides the values of the variables of
the linear equation for calculating diffusion */
pub struct DiffLinearEquationArgs {
    pub value: f64,
    pub k: f64,
}

impl DiffLinearEquationArgs {
    pub fn new(value: f64, k: f64) -> DiffLinearEquationArgs {
        DiffLinearEquationArgs { value, k }
    }
}

// Functions

/* This function estimates the values of unknowns in a set of linear equations after a
number of iterations */
pub fn gauss_seidel<T>(
    functions: Vec<GaussSeidelFunction<T>>,
    initial_values: Vec<f64>,
    iter: u8,
) -> Vec<f64> {
    let mut inital_values_clone = initial_values.clone();
    let mut iteration = 0;
    while iteration < iter {
        for (index, _) in initial_values.iter().enumerate() {
            inital_values_clone[index] = functions[index].call(&inital_values_clone);
        }
        iteration += 1;
    }
    inital_values_clone
}

/* This calculates the value of the a property after diffusion */
pub fn val_after_diff(
    surrounding_property_values: &PropertyType,
    args: &DiffLinearEquationArgs,
) -> f64 {
    (args.value
        + (args.k
            * (surrounding_property_values[0]
                + surrounding_property_values[1]
                + surrounding_property_values[2]
                + surrounding_property_values[3]))
            / 4.0)
        / (1.0 + args.k)
}

pub fn val_after_poisson(surrounding_property_values: &PropertyType, divergence: &f64) -> f64 {
    ((surrounding_property_values[0]
        + surrounding_property_values[1]
        + surrounding_property_values[2]
        + surrounding_property_values[3])
        - divergence)
        / 4.0
}

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
            $property[index] += $dt * $initial_property[index]
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
        for i in 1..$n + 1 {
            for j in 1..$n + 1 {
                let index = pure_ix_fn(i, j, $n);
                $divergence_values[index] = ($velocity_x[pure_ix_fn(i + 1, j, $n)]
                    - $velocity_x[pure_ix_fn(i - 1, j, $n)]
                    + $velocity_y[pure_ix_fn(i, j + 1, $n)]
                    - $velocity_y[pure_ix_fn(i, j - 1, $n)])
                    / 2.0
            }
        }

        set_bnd!($n, 0, $divergence_values);

        for _ in 0..10 {
            for i in 1..$n + 1 {
                for j in 1..$n + 1 {
                    let index = pure_ix_fn(i, j, $n);
                    $poisson[index] = (($poisson[pure_ix_fn(i - 1, j, $n)]
                        + $poisson[pure_ix_fn(i + 1, j, $n)]
                        + $poisson[pure_ix_fn(i, j - 1, $n)]
                        + $poisson[pure_ix_fn(i, j + 1, $n)])
                        - $divergence_values[index])
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
                    // log_f64($velocity_x[index], "velx after project");
                    $velocity_y[index] = $velocity_y[index]
                        - (($poisson[pure_ix_fn(i, j + 1, $n)])
                            - ($poisson[pure_ix_fn(i, j - 1, $n)]))
                            / 2.0;

                    // log_f64($velocity_x[index], "vely after project");
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

#[cfg(test)]
mod gauss_seidel_tests {
    use super::*;

    #[test]
    fn gauss_seidel_works() {
        fn fn1(variables: &Vec<f64>, _: &Option<u8>) -> f64 {
            (3.0 + (2.0 * variables[1]) + variables[2] + variables[3]) * (1.0 / 10.0)
        }
        let fn1_ptr: LinearEquation<Option<u8>> = fn1;
        let gauss_seidel_fn1 = GaussSeidelFunction::new(fn1_ptr, None);

        fn fn2(variables: &Vec<f64>, _: &Option<u8>) -> f64 {
            (15.0 + (2.0 * variables[0]) + variables[2] + variables[3]) * (1.0 / 10.0)
        }
        let fn2_ptr: LinearEquation<Option<u8>> = fn2;
        let gauss_seidel_fn2 = GaussSeidelFunction::new(fn2_ptr, None);

        fn fn3(variables: &Vec<f64>, _: &Option<u8>) -> f64 {
            (27.0 + variables[0] + variables[1] + variables[3]) * (1.0 / 10.0)
        }
        let fn3_ptr: LinearEquation<Option<u8>> = fn3;
        let gauss_seidel_fn3 = GaussSeidelFunction::new(fn3_ptr, None);

        fn fn4(variables: &Vec<f64>, _: &Option<u8>) -> f64 {
            ((-1.0 * 9.0) + variables[0] + variables[1] + (2.0 * variables[2])) * (1.0 / 10.0)
        }
        let fn4_ptr: LinearEquation<Option<u8>> = fn4;
        let gauss_seidel_fn4 = GaussSeidelFunction::new(fn4_ptr, None);

        let answers = gauss_seidel(
            vec![
                gauss_seidel_fn1,
                gauss_seidel_fn2,
                gauss_seidel_fn3,
                gauss_seidel_fn4,
            ],
            vec![0.0, 0.0, 0.0, 0.0],
            21,
        );
        assert_eq!(answers[0], 1.0);
        assert_eq!(answers[1], 2.0);
        assert_eq!(answers[2], 3.0);
        assert_eq!(answers[3], 0.0);
    }
}
