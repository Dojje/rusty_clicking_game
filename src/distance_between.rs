
pub fn distance_between(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let base1: i32 = x2 as i32 - x1 as i32;
    let base2: i32 = y1 as i32 - y2 as i32;

    (base1.pow(2) as f32 + base2.pow(2) as f32).sqrt()
}