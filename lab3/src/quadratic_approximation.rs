use std::{cmp::Ordering, fmt::Display};

const MAX_ITERATIONS: usize = 1000;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn compute(x: f64, calc: impl FnOnce(f64) -> f64) -> Self {
        Point { x, y: calc(x) }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point: ({:.5}, {:.5})", self.x, self.y)
    }
}

fn function(x: f64) -> f64 {
    x.powf(2_f64) + x + x.sin()
}

pub struct Input {
    pub initial: f64,
    pub delta: f64,
    pub epsilon: f64,
}

impl Input {
    pub fn solve(&self) -> f64 {
        let mut first = Point::compute(self.initial, function);

        let mut count: usize = 0;

        'second: while count < MAX_ITERATIONS {
            println!("{count:=^80}");
            println!("== SECOND STEP ==");

            let mut second = Point::compute(first.x + self.delta, function);
            let comaprison = first.y.total_cmp(&second.y);

            println!("comparison result: {comaprison:?}");

            let third = match comaprison {
                Ordering::Less | Ordering::Equal => first.x - self.delta,
                Ordering::Greater => first.x + 2.0 * self.delta,
            };

            let mut third = Point::compute(third, function);

            while count < MAX_ITERATIONS {
                count += 1;
                println!("== SIXTH STEP ==");

                println!("x1: {first}");
                println!("x2: {second}");
                println!("x3: {third}");
                println!();

                let points: [&Point; 3] = [&first, &second, &third];

                let function_minimum = points
                    .into_iter()
                    .min_by(|a, b| a.y.total_cmp(&b.y))
                    .unwrap();

                let polinom_minimum = match calculate_polinom_minimum(&points) {
                    Some(value) => value,
                    None => {
                        first = *function_minimum;
                        continue 'second;
                    }
                };
                let polinom_minimum = Point {
                    x: polinom_minimum,
                    y: function(polinom_minimum),
                };

                println!("function lowest point is: {function_minimum}");
                println!("polinom lowest point is: {polinom_minimum}");
                println!();

                let value_error =
                    ((function_minimum.y - polinom_minimum.y) / polinom_minimum.y).abs();
                let argument_error =
                    ((function_minimum.x - polinom_minimum.x) / polinom_minimum.x).abs();

                println!("value error: {:.5}", value_error);
                println!("argument_error: {:.5}", argument_error);
                println!();

                let is_value_accurate = value_error < self.epsilon;
                let is_argument_accurate = argument_error < self.epsilon;
                let is_polinom_minimum_in_range =
                    points[0].x <= polinom_minimum.x && polinom_minimum.x <= points[2].x;

                println!(
                    "polinom minimum {} within range: {} < {} < {:.5}",
                    if is_polinom_minimum_in_range {
                        ""
                    } else {
                        "NOT"
                    },
                    points[0].x,
                    polinom_minimum.x,
                    points[2].x
                );

                if is_value_accurate && is_argument_accurate {
                    return polinom_minimum.x;
                }

                if is_polinom_minimum_in_range {
                    // let lowest = *polinom_minimum.min(function_minimum);
                    let mut computed_points = [polinom_minimum, *function_minimum];
                    computed_points.sort_by(|a, b| a.y.total_cmp(&b.y));
                    let [minimum, other] = computed_points;

                    let points = [first, second, third, other];
                    let mut points_with_distance = points.map(|point| (point.x - minimum.x, point));
                    points_with_distance.sort_by(|a, b| a.0.total_cmp(&b.0));

                    first = points_with_distance
                        .into_iter()
                        .filter(|a| a.0 < 0.)
                        .next()
                        .unwrap()
                        .1;
                    second = minimum;
                    third = points_with_distance
                        .into_iter()
                        .filter(|a| a.0 > 0.)
                        .next()
                        .unwrap()
                        .1;
                } else {
                    first = polinom_minimum;
                    continue 'second;
                }
            }
        }

        panic!();
    }
}

const COEFICIENT: f64 = 0.5;

fn calculate_polinom_minimum(points: &[&Point; 3]) -> Option<f64> {
    let [first, second, third] = points;
    let denominator = (second.x - third.x) * first.y
        + (third.x - first.x) * second.y
        + (first.x - second.x) * third.y;

    if denominator == 0.0 {
        return None;
    }

    let numerator = (second.x.powi(2) - third.x.powi(2)) * first.y
        + (third.x.powi(2) - first.x.powi(2)) * second.y
        + (first.x.powi(2) - second.x.powi(2)) * third.y;

    let value = COEFICIENT * numerator / denominator;

    return Some(value);
}
