mod linear_regression;
use linear_regression::b0;
use linear_regression::b1;

fn main() {
    let x = vec![6.0, 9.5, 7.6, 6.4, 2.4, 2.4, 1.5, 9.7, 6.4, 7.4, 1.2, 9.7, 8.5, 3.0, 2.6, 2.6, 3.7, 5.7, 4.9, 3.6];
    let y = vec![21.3, 31.7, 24.0, 22.7, 9.6, 12.3, 8.0, 29.9, 19.3, 25.2, 6.0, 30.3, 25.7, 12.7, 11.2, 10.6, 14.6, 20.4, 15.5, 14.7];

    let fb1: f64 = b1(&x, &y);
    let fb0: f64 = b0(&x, &y);

    println!("y = {fb1}x + {fb0}");
}