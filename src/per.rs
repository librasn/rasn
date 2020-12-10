pub mod de;
pub mod enc;

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Aligned,
    Unaligned,
}
