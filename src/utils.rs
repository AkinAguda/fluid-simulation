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

pub type LinearEquation<T, N> = fn(variables: &Vec<T>, args: &Option<N>) -> T;

pub fn gauss_seidel<T: Clone, N>(
    functions: Vec<LinearEquation<T, N>>,
    initial_values: Vec<T>,
    iter: u16,
    function_args: Vec<Option<N>>,
) -> Vec<T> {
    let mut inital_values_clone = initial_values.clone();
    let mut iteration = 0;
    while iteration < iter {
        for (index, _) in initial_values.iter().enumerate() {
            inital_values_clone[index] =
                functions[index](&inital_values_clone, &function_args[index]);
        }
        iteration += 1;
    }
    inital_values_clone
}

pub struct DiffLinearEquationArgs {
    pub value: f64,
    pub k: f64,
}

impl DiffLinearEquationArgs {
    pub fn new(value: f64, k: f64) -> DiffLinearEquationArgs {
        DiffLinearEquationArgs { value, k }
    }
}

pub type PropertyType = Vec<f64>;

pub fn val_after_diff(
    surrounding_property_values: PropertyType,
    args: DiffLinearEquationArgs,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauss_seidel_works() {
        fn fn1(variables: &Vec<f64>) -> f64 {
            (3.0 + (2.0 * variables[1]) + variables[2] + variables[3]) / 10.0
        }
        fn fn2(variables: &Vec<f64>) -> f64 {
            (15.0 + (2.0 * variables[0]) + variables[2] + variables[3]) / 10.0
        }
        fn fn3(variables: &Vec<f64>) -> f64 {
            (27.0 + variables[0] + variables[1] + variables[3]) / 10.0
        }
        fn fn4(variables: &Vec<f64>) -> f64 {
            ((-1.0 * 9.0) + variables[0] + variables[1] + (2.0 * variables[2])) / 10.0
        }
        let answers = gauss_seidel(
            vec![fn1, fn2, fn3, fn4],
            vec![0.0, 0.0, 0.0, 0.0],
            10,
            vec![None, None, None, None],
        );
        assert_eq!(answers[0], 1.0);
        assert_eq!(answers[1], 2.0);
        assert_eq!(answers[2], 3.0);
        assert_eq!(answers[3], 0.0);
    }
}
