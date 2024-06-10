use crate::function;
use crate::gradient;
use crate::Input;
use crate::Point;
use crate::Point2D;
use crate::MAX_ITERATIONS;

pub fn gradient_method(
    Input {
        epsilon,
        first_approximation,
        iteration_step,
    }: Input,
) -> Point {
    let mut previous_function_point =
        Point::compute(first_approximation.x, first_approximation.y, function);
    println!("first approx {previous_function_point}");

    for step in 0..MAX_ITERATIONS {
        println!("{:-^80}", step + 1);
        let gradient = gradient(previous_function_point.into());
        println!("gradient {gradient}");

        let next_point = Point2D::new(
            previous_function_point.x + iteration_step * gradient.x,
            previous_function_point.y + iteration_step * gradient.y,
        );
        let next_approximation = Point::compute(next_point.x, next_point.y, function);
        println!("next approximation {next_approximation}");
        if (next_approximation.z - previous_function_point.z).abs() < epsilon {
            return next_approximation;
        }
        println!(
            "continue {}",
            (next_approximation.z - previous_function_point.z).abs()
        );

        previous_function_point = next_approximation;
    }

    panic!("Does not converges!");
}
