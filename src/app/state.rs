#[derive(Debug, Clone)]
pub enum AppState {
    Init,
    Initialized {},
}

impl AppState {
    pub fn initialized() -> Self {
        Self::Initialized {}
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
