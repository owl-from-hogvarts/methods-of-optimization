use crate::{first_derevative, function, Input, MethodError, Point, MAX_ITERATIONS};

impl Input {
    pub fn chord_method(&self) -> Result<Point, MethodError> {
        let Input {
            mut start,
            mut end,
            epsilon,
        } = *self;

        let mut count: usize = 0;
        while count < MAX_ITERATIONS {
            println!("{:-^80}", count + 1);
            let x = start
                - (first_derevative(start) / (first_derevative(start) - first_derevative(end)))
                    * (start - end);
            println!("x: {x:.5}");

            let y = first_derevative(x);
            println!("y: {y:.5}");
            if y.abs() <= epsilon {
                return Ok(Point { x, y: function(x) });
            }

            if !(start <= x && x <= end) {
                return Err(MethodError::Diverges);
            }

            if y > 0_f64 {
                println!("y > 0");
                println!("b = x");
                end = x;
            } else {
                println!("y < 0");
                println!("a = x");
                start = x;
            }

            count += 1;
        }

        return Err(MethodError::Diverges);
    }
}
