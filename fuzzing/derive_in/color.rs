#[rasn(automatic_tags)]
pub struct Color {
    #[rasn(value("0..=255"))]
    pub r: u8,
    #[rasn(value("0..=255"))]
    pub g: u8,
    #[rasn(value("0..=255"))]
    pub b: u8,
    #[rasn(value("0..=65335"))]
    pub a: u16 ,
}
