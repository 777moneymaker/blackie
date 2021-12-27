pub enum PixelStatus {
    BLVCK,
    OTHER,
}

pub fn is_tru_blvck(r: u8, g:u8, b:u8) -> PixelStatus {
    if (r, g, b) == (0, 0, 0) {
        PixelStatus::BLVCK
    } else {
        PixelStatus::OTHER
    }
}
