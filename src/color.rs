use crate::vector::Color;

pub fn to_string(c: &Color) -> String {
    String::from(format!("{} {} {}\n", (255.999 * c.x) as i32, (255.999 * c.y) as i32, (255.999 * c.z) as i32))
}
