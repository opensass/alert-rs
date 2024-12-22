use crate::common::*;
use dioxus::prelude::*;
use gloo::timers::callback::Timeout;
use web_sys::window;

/// Properties for configuring the `Alert` component.
///
/// This component supports customizable alerts with flexible behaviors, icons, buttons,
/// and styling options for use in various notification or messaging contexts.
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// The body text displayed in the alert.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub body: &'static str,

    /// A signal controlling the visibility of the alert.
    ///
    /// The alert will show or hide based on this signal.
    pub show_alert: Signal<bool>,

    /// The duration in milliseconds before the alert automatically closes.
    ///
    /// Defaults to `2500` milliseconds.
    #[props(default = 2500)]
    pub timeout: u32,

    /// The title text displayed at the top of the alert.
    ///
    /// Defaults to `"Info"`.
    #[props(default = "Info")]
    pub title: &'static str,

    /// The text displayed on the confirm button.
    ///
    /// Defaults to `"Okay"`.
    #[props(default = "Okay")]
    pub confirm_button_text: &'static str,

    /// The text displayed on the cancel button.
    ///
    /// Defaults to `"Cancel"`.
    #[props(default = "Cancel")]
    pub cancel_button_text: &'static str,

    /// Whether to show the confirm button.
    ///
    /// Defaults to `true`.
    #[props(default = true)]
    pub show_confirm_button: bool,

    /// Whether to show the cancel button.
    ///
    /// Defaults to `true`.
    #[props(default = true)]
    pub show_cancel_button: bool,

    /// Whether to show the close button.
    ///
    /// Defaults to `false`.
    #[props(default = false)]
    pub show_close_button: bool,

    /// The position of the alert on the screen.
    ///
    /// Defaults to `Position::TopRight`.
    #[props(default = Position::TopRight)]
    pub position: Position,

    /// The icon type displayed in the alert.
    ///
    /// Defaults to `IconType::Info`.
    #[props(default = IconType::Info)]
    pub icon_type: IconType,

    /// The color of the icon.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub icon_color: &'static str,

    /// The width of the icon in the alert.
    ///
    /// Defaults to `"50"`.
    #[props(default = "50")]
    pub icon_width: &'static str,

    /// Additional CSS classes for the alert container.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub alert_class: &'static str,

    /// Additional CSS classes for the icon in the alert.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub icon_class: &'static str,

    /// Additional CSS classes for the confirm button.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub confirm_button_class: &'static str,

    /// Additional CSS classes for the cancel button.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub cancel_button_class: &'static str,

    /// Additional CSS classes for the alert title.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub title_class: &'static str,

    /// Additional CSS classes for the alert body.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub body_class: &'static str,

    /// Inline styles for the alert container.
    ///
    /// Defaults to `DEFAULT_ALERT_STYLE`.
    #[props(default = DEFAULT_ALERT_STYLE)]
    pub alert_style: &'static str,

    /// Inline styles for the close button.
    ///
    /// Defaults to `DEFAULT_CLOSE_BUTTON_STYLE`.
    #[props(default = DEFAULT_CLOSE_BUTTON_STYLE)]
    pub close_button_style: &'static str,

    /// Inline styles for the confirm button.
    ///
    /// Defaults to `DEFAULT_CONFIRM_BUTTON_STYLE`.
    #[props(default = DEFAULT_CONFIRM_BUTTON_STYLE)]
    pub confirm_button_style: &'static str,

    /// Inline styles for the cancel button.
    ///
    /// Defaults to `DEFAULT_CANCEL_BUTTON_STYLE`.
    #[props(default = DEFAULT_CANCEL_BUTTON_STYLE)]
    pub cancel_button_style: &'static str,

    /// Inline styles for the icon in the alert.
    ///
    /// Defaults to `DEFAULT_ICON_STYLE`.
    #[props(default = DEFAULT_ICON_STYLE)]
    pub icon_style: &'static str,

    /// Inline styles for the alert title.
    ///
    /// Defaults to `DEFAULT_TITLE_STYLE`.
    #[props(default = DEFAULT_TITLE_STYLE)]
    pub title_style: &'static str,

    /// Inline styles for the separator between title and body.
    ///
    /// Defaults to `DEFAULT_SEPARATOR_STYLE`.
    #[props(default = DEFAULT_SEPARATOR_STYLE)]
    pub separator_style: &'static str,

    /// Inline styles for the alert body message.
    ///
    /// Defaults to `DEFAULT_MESSAGE_STYLE`.
    #[props(default = DEFAULT_MESSAGE_STYLE)]
    pub message_style: &'static str,

    /// Whether to use the native browser alert.
    ///
    /// Defaults to `false`.
    #[props(default = false)]
    pub native: bool,

    /// Callback triggered before the alert opens.
    ///
    /// Defaults to an empty callback.
    #[props(default)]
    pub will_open: Callback<()>,

    /// Callback triggered after the alert opens.
    ///
    /// Defaults to an empty callback.
    #[props(default)]
    pub did_open: Callback<()>,

    /// Callback triggered after the alert closes.
    ///
    /// Defaults to an empty callback.
    #[props(default)]
    pub did_close: Callback<()>,

    /// Callback triggered when the confirm button is clicked.
    ///
    /// Defaults to an empty callback.
    #[props(default)]
    pub on_confirm: Callback<()>,

    /// Callback triggered when the close button is clicked.
    ///
    /// Defaults to an empty callback.
    #[props(default)]
    pub on_close: Callback<()>,

    /// Callback triggered when the cancel button is clicked.
    ///
    /// Defaults to an empty callback.
    #[props(default)]
    pub on_cancel: Callback<()>,
}

/// Alert Component
///
/// A Dioxus component for displaying customizable alerts with a range of styling and behavioral options.
/// This `Alert` component allows you to show notifications, warnings, or messages with tailored
/// icons, buttons, and positioning. It is a highly flexible solution for user-facing alerts.
///
/// # Properties
/// The component uses the `AlertProps` struct for configuration. Key properties include:
///
/// - **body**: The text content of the alert message (`&'static str`). Default: `""`.
/// - **show_alert**: A `Signal<bool>` controlling the alert's visibility. This is a required prop.
/// - **timeout**: The duration in milliseconds before the alert auto-closes (`u32`). Default: `2500`.
/// - **title**: The heading text for the alert (`&'static str`). Default: `"Info"`.
/// - **confirm_button_text**: The text for the confirm button (`&'static str`). Default: `"Okay"`.
/// - **cancel_button_text**: The text for the cancel button (`&'static str`). Default: `"Cancel"`.
/// - **show_confirm_button**: Determines whether the confirm button is visible (`bool`). Default: `true`.
/// - **show_cancel_button**: Determines whether the cancel button is visible (`bool`). Default: `true`.
/// - **show_close_button**: Determines whether a close button is included (`bool`). Default: `false`.
/// - **position**: The screen position of the alert (`Position`). Default: `Position::TopRight`.
/// - **icon_type**: The type of icon to display (`IconType`). Default: `IconType::Info`.
/// - **icon_color**: CSS color for the alert icon (`&'static str`). Default: `""`.
/// - **icon_width**: Width of the alert icon (`&'static str`). Default: `"50"`.
/// - **alert_class**: Custom CSS class for the alert container (`&'static str`). Default: `""`.
/// - **icon_class**: Custom CSS class for the alert icon (`&'static str`). Default: `""`.
/// - **confirm_button_class**: Custom CSS class for the confirm button (`&'static str`). Default: `""`.
/// - **cancel_button_class**: Custom CSS class for the cancel button (`&'static str`). Default: `""`.
/// - **title_class**: Custom CSS class for the alert title (`&'static str`). Default: `""`.
/// - **body_class**: Custom CSS class for the alert body (`&'static str`). Default: `""`.
/// - **alert_style**: Inline style for the alert component (`&'static str`). Default: `DEFAULT_ALERT_STYLE`.
/// - **close_button_style**: Inline style for the close button (`&'static str`). Default: `DEFAULT_CLOSE_BUTTON_STYLE`.
/// - **confirm_button_style**: Inline style for the confirm button (`&'static str`). Default: `DEFAULT_CONFIRM_BUTTON_STYLE`.
/// - **cancel_button_style**: Inline style for the cancel button (`&'static str`). Default: `DEFAULT_CANCEL_BUTTON_STYLE`.
/// - **icon_style**: Inline style for the alert icon (`&'static str`). Default: `DEFAULT_ICON_STYLE`.
/// - **title_style**: Inline style for the alert title (`&'static str`). Default: `DEFAULT_TITLE_STYLE`.
/// - **separator_style**: Inline style for the separator line (`&'static str`). Default: `DEFAULT_SEPARATOR_STYLE`.
/// - **message_style**: Inline style for the alert message text (`&'static str`). Default: `DEFAULT_MESSAGE_STYLE`.
/// - **native**: If `true`, uses the browser's native alert instead of the custom component (`bool`). Default: `false`.
/// - **will_open**: Callback invoked before the alert is displayed (`Callback<()>`). Default: no-op.
/// - **did_open**: Callback invoked after the alert is displayed (`Callback<()>`). Default: no-op.
/// - **did_close**: Callback invoked after the alert is closed (`Callback<()>`). Default: no-op.
/// - **on_confirm**: Callback invoked when the confirm button is clicked (`Callback<()>`). Default: no-op.
/// - **on_close**: Callback invoked when the close button is clicked (`Callback<()>`). Default: no-op.
/// - **on_cancel**: Callback invoked when the cancel button is clicked (`Callback<()>`). Default: no-op.
///
/// # Features
/// - Highly customizable appearance and behavior.
/// - Supports dynamic positioning and icon configuration.
/// - Configurable buttons for confirmation, cancellation, and closing.
/// - Optional timeout for auto-closing the alert.
/// - Native or custom alert rendering options.
/// - Built-in callback support for interactive handling of user actions.
///
/// # Examples
///
/// ## Basic Alert
/// ```rust
/// use dioxus::prelude::*;
/// use alert_rs::dioxus::Alert;
///
/// fn App() -> Element {
///     let mut show_alert = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| show_alert.set(true),
///             "Show Alert"
///         }
///         Alert {
///             show_alert: show_alert.clone(),
///             title: "Hello",
///             body: "This is a basic alert.",
///         }
///     }
/// }
/// ```
///
/// ## Customized Alert with Buttons
/// ```rust
/// use dioxus::prelude::*;
/// use alert_rs::dioxus::Alert;
///
/// fn App() -> Element {
///     let mut show_alert = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| show_alert.set(true),
///             "Custom Alert"
///         }
///         Alert {
///             show_alert: show_alert.clone(),
///             title: "Alert Title",
///             body: "This is a custom alert with buttons.",
///             confirm_button_text: "Confirm",
///             cancel_button_text: "Cancel",
///             on_confirm: |_| println!("Confirmed!"),
///             on_cancel: |_| println!("Cancelled!"),
///         }
///     }
/// }
/// ```
///
/// ## Native Alert
/// ```rust
/// use dioxus::prelude::*;
/// use alert_rs::dioxus::Alert;
///
/// fn App() -> Element {
///     let mut show_alert = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| show_alert.set(true),
///             "Native Alert"
///         }
///         Alert {
///             show_alert: show_alert.clone(),
///             native: true,
///             body: "This uses the browser's native alert.",
///         }
///     }
/// }
/// ```
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let timeout = props.timeout;
    let native = props.native;
    let mut show_alert = props.show_alert;

    let title = props.title.to_string();
    let body = props.body.to_string();

    let show_confirm_button = props.show_confirm_button;
    let show_cancel_button = props.show_cancel_button;
    let show_close_button = props.show_close_button;
    let icon_color = props.icon_color;
    let icon_type = props.icon_type;
    let icon_width = props.icon_width;
    let icon_style = props.icon_style;

    use_effect(move || {
        if show_alert() && !native {
            props.will_open.call(());

            let _ = Timeout::new(timeout, move || {
                show_alert.set(false);
                props.did_close.call(());
            })
            .forget();

            props.did_open.call(());
        } else if show_alert() && native {
            if let Some(win) = window() {
                props.will_open.call(());

                let full_message = if !title.is_empty() {
                    format!("{}\n\n{}", title, body)
                } else {
                    body.clone()
                };

                match (show_confirm_button, show_cancel_button, show_close_button) {
                    (true, true, true) => {
                        if win.confirm_with_message(&full_message).unwrap_or(false) {
                            props.on_confirm.call(());
                        } else {
                            props.on_close.call(());
                            props.on_cancel.call(());
                        }
                    }
                    (true, true, false) => {
                        if win.confirm_with_message(&full_message).unwrap_or(false) {
                            props.on_confirm.call(());
                        } else {
                            props.on_cancel.call(());
                        }
                    }
                    (true, false, false) => {
                        win.alert_with_message(&full_message).ok();
                        props.on_confirm.call(());
                    }
                    _ => {}
                }

                Timeout::new(timeout, move || {
                    show_alert.set(false);
                    props.did_close.call(());
                })
                .forget();

                props.did_open.call(());
            }
        }
    });

    let position_style = match props.position {
        Position::TopLeft => "top: 0; left: 0;",
        Position::TopCenter => "top: 0; left: 50%; transform: translateX(-50%);",
        Position::TopRight => "top: 0; right: 0;",
        Position::LeftCenter => "top: 50%; left: 0; transform: translateY(-50%);",
        Position::Center => "top: 50%; left: 50%; transform: translate(-50%, -50%);",
        Position::BottomCenter => "bottom: 0; left: 50%; transform: translateX(-50%);",
        Position::RightCenter => "top: 50%; right: 0; transform: translateY(-50%);",
        Position::BottomRight => "bottom: 0; right: 0;",
        Position::BottomLeft => "bottom: 0; left: 0;",
        Position::Custom(x, y) => &format!("top: {}; left: {};", y, x),
    };

    let icon_color = if icon_color.is_empty() {
        match icon_type {
            IconType::Warning => "orange",
            IconType::Error => "red",
            IconType::Success => "green",
            IconType::Info => "blue",
            IconType::Question => "gray",
        }
    } else {
        props.icon_color
    };

    // SVGs taken from: https://fontawesome.com/icons
    let icon_tag = match icon_type {
        IconType::Warning => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{icon_width}",
                style: "{icon_style}",
                class: "p-2 m-2",
                fill: "{icon_color}",
                view_box: "0 0 512 512",
                path {
                    d: "M248.4 84.3c1.6-2.7 4.5-4.3 7.6-4.3s6 1.6 7.6 4.3L461.9 410c1.4 2.3 2.1 4.9 2.1 7.5c0 8-6.5 14.5-14.5 14.5H62.5c-8 0-14.5-6.5-14.5-14.5c0-2.7 .7-5.3 2.1-7.5L248.4 84.3zm-41-25L9.1 385c-6 9.8-9.1 21-9.1 32.5C0 452 28 480 62.5 480h387c34.5 0 62.5-28 62.5-62.5c0-11.5-3.2-22.7-9.1-32.5L304.6 59.3C294.3 42.4 275.9 32 256 32s-38.3 10.4-48.6 27.3zM288 368a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm-8-184c0-13.3-10.7-24-24-24s-24 10.7-24 24v96c0 13.3 10.7 24 24 24s24-10.7 24-24V184z"
                }
            }
        },
        IconType::Error => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{icon_width}",
                style: "{icon_style}",
                class: "p-2 m-2",
                fill: "{icon_color}",
                view_box: "0 0 20 20",
                path {
                    d: "M12.71,7.291c-0.15-0.15-0.393-0.15-0.542,0L10,9.458L7.833,7.291c-0.15-0.15-0.392-0.15-0.542,0c-0.149,0.149-0.149,0.392,0,0.541L9.458,10l-2.168,2.167c-0.149,0.15-0.149,0.393,0,0.542c0.15,0.149,0.392,0.149,0.542,0L10,10.542l2.168,2.167c0.149,0.149,0.392,0.149,0.542,0c0.148-0.149,0.148-0.392,0-0.542L10.542,10l2.168-2.168C12.858,7.683,12.858,7.44,12.71,7.291z M10,1.188c-4.867,0-8.812,3.946-8.812,8.812c0,4.867,3.945,8.812,8.812,8.812s8.812-3.945,8.812-8.812C18.812,5.133,14.867,1.188,10,1.188z M10,18.046c-4.444,0-8.046-3.603-8.046-8.046c0-4.444,3.603-8.046,8.046-8.046c4.443,0,8.046,3.602,8.046,8.046C18.046,14.443,14.443,18.046,10,18.046z"
                }
            }
        },
        IconType::Success => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{icon_width}",
                style: "{icon_style}",
                class: "p-2 m-2",
                fill: "{icon_color}",
                view_box: "0 0 512 512",
                path {
                    d: "M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM369 209c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-111 111-47-47c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9l64 64c9.4 9.4 24.6 9.4 33.9 0L369 209z"
                }
            }
        },
        IconType::Info => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{icon_width}",
                style: "{icon_style}",
                class: "p-2 m-2",
                fill: "{icon_color}",
                view_box: "0 0 16 16",
                path { d: "M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z" }
                path {
                    d: "m8.93 6.588-2.29.287-.082.38.45.083c.294.07.352.176.288.469l-.738 3.468c-.194.897.105 1.319.808 1.319.545 0 1.178-.252 1.465-.598l.088-.416c-.2.176-.492.246-.686.246-.275 0-.375-.193-.304-.533L8.93 6.588zM9 4.5a1 1 0 1 1-2 0 1 1 0 0 1 2 0z"
                }
            }
        },
        IconType::Question => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{icon_width}",
                style: "{icon_style}",
                class: "p-2 m-2",
                fill: "{icon_color}",
                view_box: "0 0 16 16",
                path { d: "M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z" }
                path {
                    d: "M5.255 5.786a.237.237 0 0 0 .241.247h.825c.138 0 .248-.113.266-.25.09-.656.54-1.134 1.342-1.134.686 0 1.314.343 1.314 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.003.217a.25.25 0 0 0 .25.246h.811a.25.25 0 0 0 .25-.25v-.105c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.267 0-2.655.59-2.75 2.286zm1.557 5.763c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94z"
                }
            }
        },
    };

    let on_cancel = {
        move |_| {
            props.on_cancel.call(());
            show_alert.set(false);
        }
    };

    let on_confirm = {
        move |_| {
            props.on_confirm.call(());
        }
    };

    if !native {
        rsx! {
            if show_alert() {
                div {
                    style: props.alert_style,
                    div {
                        class: props.alert_class,
                        style: format!("position: absolute; {}", position_style),
                        if show_close_button {
                            button {
                                style: props.close_button_style,
                                onclick: move |_| {
                                    props.on_cancel.call(());
                                    show_alert.set(false);
                                },
                                "X"
                            }
                        }
                        div {
                            class: props.icon_class,
                            style: props.icon_style,
                            {icon_tag}
                        }
                        strong {
                            class: props.title_class,
                            style: props.title_style,
                            "{props.title}"
                        }
                        hr { style: props.separator_style }
                        p {
                            class: props.body_class,
                            style: props.message_style,
                            "{props.body}"
                        }
                        if props.show_confirm_button {
                            button {
                                class: props.confirm_button_class,
                                style: props.confirm_button_style,
                                onclick: on_confirm,
                                "{props.confirm_button_text}"
                            }
                        }
                        if props.show_cancel_button {
                            button {
                                class: props.cancel_button_class,
                                style: props.cancel_button_style,
                                onclick:on_cancel,
                                "{props.cancel_button_text}"
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
