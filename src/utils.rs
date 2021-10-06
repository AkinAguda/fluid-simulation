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

type LinearEquation<T> = fn(variables: &Vec<T>) -> T;

pub fn gauss_seidel<T: Clone>(
    functions: Vec<LinearEquation<T>>,
    initial_values: Vec<T>,
    iter: u16,
) -> Vec<T> {
    let mut inital_values_clone = initial_values.clone();
    let mut iteration = 0;
    while iteration < iter {
        for (index, _) in initial_values.iter().enumerate() {
            inital_values_clone[index] = functions[index](&inital_values_clone);
        }
        iteration += 1;
    }
    inital_values_clone
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauss_seidel_works() {
        fn fn1(variables: &Vec<f32>) -> f32 {
            (3.0 + (2.0 * variables[1]) + variables[2] + variables[3]) / 10.0
        }
        fn fn2(variables: &Vec<f32>) -> f32 {
            (15.0 + (2.0 * variables[0]) + variables[2] + variables[3]) / 10.0
        }
        fn fn3(variables: &Vec<f32>) -> f32 {
            (27.0 + variables[0] + variables[1] + variables[3]) / 10.0
        }
        fn fn4(variables: &Vec<f32>) -> f32 {
            ((-1.0 * 9.0) + variables[0] + variables[1] + (2.0 * variables[2])) / 10.0
        }
        let answers = gauss_seidel(vec![fn1, fn2, fn3, fn4], vec![0.0, 0.0, 0.0, 0.0], 10);
        assert_eq!(answers[0], 1.0);
        assert_eq!(answers[1], 2.0);
        assert_eq!(answers[2], 3.0);
        assert_eq!(answers[3], 0.0);
    }
}
