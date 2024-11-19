pub trait ToSample {
    fn to_sample(self) -> f32;
}

impl ToSample for u16 {
    fn to_sample(self) -> f32 {
        self as f32 / 65535.0 - 0.5
    }
}
