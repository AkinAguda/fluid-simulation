// pub fn set_panic_hook() {
//     // When the `console_error_panic_hook` feature is enabled, we can call the
//     // `set_panic_hook` function at least once during initialization, and then
//     // we will get better error messages if our code ever panics.
//     //
//     // For more details see
//     // https://github.com/rustwasm/console_error_panic_hook#readme
//     #[cfg(feature = "console_error_panic_hook")]
//     console_error_panic_hook::set_once();
// }

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

#[macro_export]
macro_rules! add_source {
    ($property:expr, $initial_property:expr, $size:expr, $dt:expr) => {
        for index in 0..$size {
            $property[index] += $dt * $initial_property[index]
        }
    };
}

// #[macro_export]
// macro_rules! swap {
//     ($vector_1, $vector_2) => {

//     };
// }

#[cfg(test)]
mod tests {
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
