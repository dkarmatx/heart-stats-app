mod home;
mod input;

pub use home::HomePanel;
pub use input::InputPanel;

pub enum PanelError {
    Error(String),
    FatalError(String),
}

impl PanelError {
    pub fn desc(&self) -> String {
        match self {
            Self::Error(desc) => t!("app.errors.error_template", desc = desc.as_str()),
            Self::FatalError(desc) => t!("app.errors.fatal_error_template", desc = desc.as_str()),
        }
        .into()
    }

    pub fn is_fatal(&self) -> bool {
        match self {
            Self::FatalError(_) => true,
            _ => false,
        }
    }
}

pub type Result<T> = std::result::Result<T, PanelError>;
