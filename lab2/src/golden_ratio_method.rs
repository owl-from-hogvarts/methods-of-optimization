use crate::{function, Input, MethodError, Point, MAX_ITERATIONS};

impl Input {
    pub fn golden_ration_method(&self) -> Result<Point, MethodError> {
        let Input {
            mut start,
            mut end,
            epsilon,
        } = *self;

        let mut next_x1 = calculate_start(start, end);
        let mut next_x2 = calculate_end(start, end);

        let mut count: usize = 0;
        loop {
            println!("{:-^80}", count + 1);
            let length = end - start;
            let x1 = next_x1;
            let x2 = next_x2;
            let y1 = function(x1);
            let y2 = function(x2);
            println!("x1: {x1:.5}");
            println!("x2: {x2:.5}");
            println!("y1: {y1:.5}");
            println!("y2: {y2:.5}");

            if y1 < y2 {
                println!("y1 < y2");
                end = x2;
                next_x1 = calculate_start(start, end);
                next_x2 = x1;
            } else {
                println!("y1 > y2");
                start = x1;
                next_x1 = x2;
                next_x2 = calculate_end(start, end);
            }

            let new_length = end - start;

            if new_length < epsilon {
                // if y1 > y2 {
                //     return Ok(Point { x: x1, y: y1 });
                // }

                // return Ok(Point { x: x2, y: y2 });
                let final_x = (start + end) / 2_f64;
                return Ok(Point {
                    x: final_x,
                    y: function(final_x),
                });
            }

            if new_length >= length || count >= MAX_ITERATIONS {
                return Err(MethodError::Diverges);
            }

            count += 1;
        }
    }
}

fn calculate_start(start: f64, end: f64) -> f64 {
    start + 0.382_f64 * (end - start)
}

fn calculate_end(start: f64, end: f64) -> f64 {
    start + 0.618_f64 * (end - start)
}
