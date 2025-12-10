use num::pow;

fn main() {

    let a: i32 = 1;
    let b: i32 = 6;
    let c: i32 = 9;

    let x: f64 = quadratic(a, b, c);

    println!("The zero of {}x^2 + {}x + {} is {}", a, b, c, x);
}

fn quadratic(a: i32, b: i32, c: i32) -> f64 {
    ((-b + float.sqrt(pow(b, 2) - (4*a*c))) / (2*a)).into()
}