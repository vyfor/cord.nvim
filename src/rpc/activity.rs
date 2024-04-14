#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Activity {
    pub details: Option<String>,
    pub state: Option<String>,
    pub assets: Option<ActivityAssets>,
    pub buttons: Option<Vec<ActivityButton>>,
    pub timestamp: Option<u128>,
}

impl Default for Activity {
    fn default() -> Self {
        Activity {
            details: None,
            state: None,
            assets: None,
            buttons: None,
            timestamp: None,
        }
    }
}