/// Alert icon types
#[derive(Debug, PartialEq, Clone, Default)]
pub enum IconType {
    Warning,
    Error,
    Success,
    Info,
    #[default]
    Question,
}

/// Alert positions
#[derive(Debug, PartialEq, Clone, Default)]
pub enum Position {
    TopLeft,
    TopCenter,
    TopRight,
    LeftCenter,
    #[default]
    Center,
    RightCenter,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom(&'static str, &'static str),
}

/// Default styles
pub const DEFAULT_ALERT_STYLE: &str =
    "position: fixed; width: 100vw; height: 100vh; top: 0; left: 0; background: rgba(0, 0, 0, 0.75); z-index: 10; display: flex; justify-content: center; align-items: center;";
pub const DEFAULT_CLOSE_BUTTON_STYLE: &str = "position: absolute; top: 10px; right: 10px;";
pub const DEFAULT_CONFIRM_BUTTON_STYLE: &str =
    "margin: 5px; padding: 5px 10px; background-color: green; color: white; border: none; border-radius: 5px;";
pub const DEFAULT_CANCEL_BUTTON_STYLE: &str =
    "margin: 5px; padding: 5px 10px; background-color: red; color: white; border: none; border-radius: 5px;";
pub const DEFAULT_ICON_STYLE: &str =
    "display: flex; justify-content: center; align-items: center; padding: 2px; margin: 2px;";
pub const DEFAULT_TITLE_STYLE: &str =
    "justify-content: center; align-items: center; font-size: 26px;";
pub const DEFAULT_SEPARATOR_STYLE: &str = "margin: 10px 0;";
pub const DEFAULT_MESSAGE_STYLE: &str = "font-size: 14px;";
