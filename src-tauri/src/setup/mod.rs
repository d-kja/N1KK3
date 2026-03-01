use std::fmt::Display;

pub struct Setup {}

#[derive(Debug, Clone)]
pub enum SetupError {
    UnableToSetup(String)
}

impl Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::UnableToSetup(item) => write!(f, "Unable to setup {}", item),
        }
    }
}

impl Setup {
    pub fn run() -> Result<(), SetupError> {
        // TOOD: finish this shit
        
        Ok(())
    }
}
