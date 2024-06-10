use crate::{
    first_derevative, function, second_derevative, Input, MethodError, Point, MAX_ITERATIONS,
};

impl Input {
    pub fn newton_method(&self) -> Result<Point, MethodError> {
        let Input {
            start,
            end,
            epsilon,
        } = *self;

        let x_0 = (start + end) / 2_f64;
        let mut next_x = x_0;

        let mut count = 0;
        while count <= MAX_ITERATIONS {
            println!("{:-^80}", count + 1);
            let current_x = next_x;
            let first_derevative_value = first_derevative(current_x);
            println!("first derevative value: {first_derevative_value:.5} > epsilon");

            if first_derevative_value.abs() <= epsilon {
                return Ok(Point {
                    x: current_x,
                    y: function(current_x),
                });
            }

            next_x = current_x - (first_derevative_value / second_derevative(current_x));
            println!("x: {next_x:.5}");
            if !(start <= next_x && next_x <= end) {
                return Err(MethodError::Diverges);
            }
            count += 1;
        }

        Err(MethodError::Diverges)
    }
}
