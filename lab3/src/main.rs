mod quadratic_approximation;

use quadratic_approximation::Input;

fn main() {
    let input = Input {
        initial: 1.,
        delta: 0.1,
        epsilon: 0.0001,
    };

    println!("{}", input.solve());
}
