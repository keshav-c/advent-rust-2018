pub struct Claim {
    id: String,
    pub coordinates: (i32, i32),
    width: i32,
    height: i32,
}

impl Claim {
    pub fn new(id: &str, coordinates: (i32, i32), width: i32, height: i32) -> Self {
        Self {
            id: id.to_owned(),
            coordinates,
            width,
            height,
        }
    }
}

// impl TryFrom<&str> for Claim {
//     type Error = ();

//     fn try_from(value: &str) -> Result<Self, Self::Error> {}
// }
