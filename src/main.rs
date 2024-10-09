fn main() {
    let sixteenth = 0.0625;
    let mut current: f64 = 0.0;
    for n in 1..16 + 1 {
        current += sixteenth;
        println!("{n}/16: {current}");
    }
}
