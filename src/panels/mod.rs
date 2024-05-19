mod home;
mod input;

pub use home::HomePanel;
pub use input::InputPanel;


pub enum PanelError {
    Error(String),
    FatalError(String),
}

impl PanelError {
    pub fn desc(&self, lang: &str) -> String {
        match self {
            Self::Error(desc) => {
                locales::t!("app.errors.error_template", desc: desc.as_str(), lang)
            },
            Self::FatalError(desc) => {
                locales::t!("app.errors.fatal_error_template", desc: desc.as_str(), lang)
            },
        }
    }

    pub fn is_fatal(&self) -> bool {
        match self {
            Self::FatalError(_) => true,
            _ => false,
        }
    }
}

pub type Result<T> = std::result::Result<T, PanelError>;
