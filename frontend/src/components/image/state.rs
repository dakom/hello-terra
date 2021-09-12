#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Image {
    pub kind: ImageKind
}

impl Image {
    pub fn new(kind: ImageKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageKind {
    TerraLogo
}


impl ImageKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            TerraLogo => "terra-logo.png"
        }
    }
}
