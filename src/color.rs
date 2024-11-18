use crate::vec3::Vec3;

pub(crate) type Color = Vec3;

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Color::new(r, g, b)
    }
    pub fn r(&self) -> f64 {self.x()}
    pub fn g(&self) -> f64 {self.y()}
    pub fn b(&self) -> f64 {self.z()}

}

pub fn write_color(color: Color) {
    let rbyte = (color.r() * 255.999) as u8;
    let gbyte = (color.g() * 255.999) as u8;
    let bbyte = (color.b() * 255.999) as u8;
    println!("{} {} {}", rbyte, gbyte, bbyte);
}