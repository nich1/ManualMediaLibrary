#[derive(Debug)]
pub enum ValidationError {
    EmptyTitle,
}

#[derive(Debug, Clone)]
pub struct MediaTitle(String);

impl MediaTitle {
    pub fn new(value: String) -> Result<Self, ValidationError> {
        if value.trim().is_empty() {
            return Err(ValidationError::EmptyTitle);
        }

        Ok(Self(value))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}