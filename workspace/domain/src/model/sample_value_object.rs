pub struct SampleValueObject {
    value: String,
}

impl SampleValueObject {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}
