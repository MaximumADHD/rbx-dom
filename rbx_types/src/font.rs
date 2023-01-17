use crate::Content;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Heavy = 900,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FontStyle {
    Normal,
    Italic
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontFace {
    pub family: Content,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub cached_face_id: Content,
}

impl FontFace {
    pub fn new(family: Content, weight: FontWeight, style: FontStyle, cached_face_id: Content) -> Self {
        Self { family, weight, style, cached_face_id }
    }
}