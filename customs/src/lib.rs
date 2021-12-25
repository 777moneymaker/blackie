pub mod extensions {
    pub fn ends_with_custom(s: &str) -> bool {
        let suffixes = vec![".jpg", ".jpeg", ".png"];
        return suffixes.iter().any(|&suffix| s.ends_with(suffix));
    }

    pub fn get_suffix(s: &str) -> (&str, &str) {
        if s.ends_with(".jpg") {
            (s, ".jpg")
        } else if s.ends_with(".jpeg") {
            (s, ".jpeg")
        } else {
            (s, ".png")
        }
    }
}

pub mod pixel {
    const BLVCK: bool = true;
    const OTHER: bool = false;

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
}
