use crate::{function, Input, MethodError, Point, MAX_ITERATIONS};

impl Input {
    pub fn halving_method(&self) -> Result<Point, MethodError> {
        let Input {
            mut start,
            mut end,
            epsilon,
        } = *self;

        let mut count: usize = 0;
        while (end - start) > (2_f64 * epsilon) {
            println!("interval length: {}", end - start);
            println!("{:-^80}", count + 1);
            println!("start, end: {start}, {end}");
            if count >= MAX_ITERATIONS {
                return Err(MethodError::Diverges);
            }

            let x1: f64 = (start + end - epsilon) / 2_f64;
            let x2: f64 = (start + end + epsilon) / 2_f64;
            let y1 = function(x1);
            let y2 = function(x2);
            println!("x1: {x1}");
            println!("x2: {x2}");
            println!("y1: {y1}");
            println!("y2: {y2}");

            if y1 > y2 {
                start = x1;
                println!("y1 > y2");
            } else {
                end = x2;
                println!("y1 < y2");
            }

            count += 1;
        }

        let xm = (start + end) / 2_f64;
        let ym = function(xm);

        Ok(Point { x: xm, y: ym })
    }
}
