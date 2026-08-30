fn main() {
    let calc:f64 = mars_weight_calculater(51.0);
    println!("{:?}",calc);
}

fn mars_weight_calculater(weight:f64) -> f64 {
    weight * 0.378
}
