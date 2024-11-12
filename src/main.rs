fn main() {
    let x : f64 = 5.0;
    let y : f64 = 10.0;
    let z : f64 = average(x as f64, y as f64);
    println!("The average of {} and {} is {}", x, y, z);
}

// Average
fn average(float1 : f64, float2 : f64) -> f64 {
    (float1 + float2) / 2.0
}

//

