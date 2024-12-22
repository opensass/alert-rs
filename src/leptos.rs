use crate::common::*;
use leptos::{ev::MouseEvent, prelude::*, *};
use std::time::Duration;
use web_sys::window;

/// Alert Component
///
/// A Leptos component for displaying customizable alerts with various behaviors and styling options.
/// This `Alert` component supports features like setting alert visibility, timeout durations, custom
/// icons, button visibility, alert positioning, and more. It can be used to display messages, warnings,
/// or notifications with a variety of visual elements and actions.
///
/// # Properties
/// The component uses the `AlertProps` struct for its properties. Key properties include:
///
/// - **show_alert**: State handle controlling the visibility of the alert (`Signal<bool>`). This is a required prop to manage alert visibility.
/// - **body**: The content of the alert message (`&'static str`). Default: `""`.
/// - **title**: The title text of the alert (`&'static str`). Default: `"Info"`.
/// - **timeout**: Timeout duration in milliseconds before the alert auto-closes (`u32`). Default: `2500`.
/// - **confirm_button_text**: Text for the confirm button (`&'static str`). Default: `"Okay"`.
/// - **cancel_button_text**: Text for the cancel button (`&'static str`). Default: `"Cancel"`.
/// - **show_confirm_button**: Whether to display the confirm button (`bool`). Default: `true`.
/// - **show_cancel_button**: Whether to display the cancel button (`bool`). Default: `true`.
/// - **show_close_button**: Whether to display the close button (`bool`). Default: `false`.
/// - **position**: The position of the alert on the screen (`Position`). Default: `Position::TopRight`.
/// - **icon_type**: The icon type displayed with the alert (`IconType`). Default: `IconType::Info`.
/// - **icon_color**: The color of the icon (`&'static str`). Default: `""`.
/// - **icon_width**: The width of the icon (`&'static str`). Default: `"50"`.
/// - **alert_class**: CSS class for styling the alert container (`&'static str`). Default: `""`.
/// - **icon_class**: CSS class for styling the icon (`&'static str`). Default: `""`.
/// - **confirm_button_class**: CSS class for styling the confirm button (`&'static str`). Default: `""`.
/// - **cancel_button_class**: CSS class for styling the cancel button (`&'static str`). Default: `""`.
/// - **title_class**: CSS class for styling the alert title (`&'static str`). Default: `""`.
/// - **body_class**: CSS class for styling the message text in the alert (`&'static str`). Default: `""`.
/// - **alert_style**: Default inline styles for the alert (`&'static str`). Default: `DEFAULT_ALERT_STYLE`.
/// - **close_button_style**: Default inline styles for the close button (`&'static str`). Default: `DEFAULT_CLOSE_BUTTON_STYLE`.
/// - **confirm_button_style**: Default inline styles for the confirm button (`&'static str`). Default: `DEFAULT_CONFIRM_BUTTON_STYLE`.
/// - **cancel_button_style**: Default inline styles for the cancel button (`&'static str`). Default: `DEFAULT_CANCEL_BUTTON_STYLE`.
/// - **icon_style**: Default inline styles for the icon (`&'static str`). Default: `DEFAULT_ICON_STYLE`.
/// - **title_style**: Default inline styles for the title text (`&'static str`). Default: `DEFAULT_TITLE_STYLE`.
/// - **separator_style**: Default inline styles for the separator (`&'static str`). Default: `DEFAULT_SEPARATOR_STYLE`.
/// - **message_style**: Default inline styles for the message text (`&'static str`). Default: `DEFAULT_MESSAGE_STYLE`.
/// - **native**: Whether to use the native alert implementation (`bool`). Default: `false`.
/// - **will_open**: Callback triggered before the alert opens (`Callback<()>`). Default: no-op.
/// - **did_open**: Callback triggered after the alert opens (`Callback<()>`). Default: no-op.
/// - **did_close**: Callback triggered after the alert closes (`Callback<()>`). Default: no-op.
/// - **on_confirm**: Callback triggered when the confirm button is clicked (`Callback<()>`). Default: no-op.
/// - **on_close**: Callback triggered when the close button is clicked (`Callback<()>`). Default: no-op.
/// - **on_cancel**: Callback triggered when the cancel button is clicked (`Callback<()>`). Default: no-op.
///
/// # Features
/// - Customizable alert message and title.
/// - Configurable buttons (confirm, cancel, and close).
/// - Adjustable timeout duration for auto-closing.
/// - Dynamic positioning and icon customization.
/// - Supports native browser alert functionality with `native`.
/// - Callbacks for various actions, such as opening, closing, and button clicks.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use leptos::prelude::*;
/// use alert_rs::leptos::Alert;
///
/// #[component]
/// pub fn MyComponent() -> impl IntoView {
///     let show_alert = create_signal(false);
///     view! {
///         <>
///             <button on:click={move |_| show_alert.1.set(true)}>{ "Show Alert" }</button>
///             <Alert show_alert={show_alert} />
///         </>
///     }
/// }
/// ```
///
/// ## Custom Alert with Buttons
/// ```rust
/// use leptos::prelude::*;
/// use alert_rs::leptos::Alert;
///
/// #[component]
/// pub fn CustomAlert() -> impl IntoView {
///     let show_alert = create_signal(false);
///     view! {
///         <>
///             <button on:click={move |_| show_alert.1.set(true)}>{ "Show Custom Alert" }</button>
///             <Alert
///                 show_alert={show_alert}
///                 body="This is a custom alert message!"
///                 title="Custom Alert"
///                 confirm_button_text="Yes"
///                 cancel_button_text="No"
///                 on_confirm={Callback::from(move || log::info!("Confirmed!"))}
///                 on_cancel={Callback::from(move || log::info!("Cancelled!"))}
///             />
///         </>
///     }
/// }
/// ```
///
/// ## Native Alert
/// ```rust
/// use leptos::prelude::*;
/// use alert_rs::leptos::Alert;
///
/// #[component]
/// pub fn NativeAlert() -> impl IntoView {
///     let show_alert = create_signal(false);
///     view! {
///         <>
///             <button on:click={move |_| show_alert.1.set(true)}>{ "Show Native Alert" }</button>
///             <Alert show_alert={show_alert} native={true} />
///         </>
///     }
/// }
/// ```
///
/// # Behavior
/// - The component uses `create_signal` to manage the visibility of the alert.
/// - It also uses `create_effect` to handle side-effects such as displaying the alert and auto-closing after the specified timeout.
/// - Buttons trigger specific callbacks (e.g., confirm, cancel, close) when clicked.
///
/// # Notes
/// - The `native` prop can be set to `true` to use the browser's default alert behavior instead of the custom component.
#[component]
pub fn Alert(
    /// The body text of the alert.
    ///
    /// This defines the message content that will be displayed within the alert.
    /// Defaults to an empty string if not specified.
    #[prop(default = "")]
    body: &'static str,

    /// The state handle controlling the visibility of the alert.
    ///
    /// This should be used to manage whether the alert is shown or not. It is a required prop.
    show_alert: (
        ReadSignal<bool>,
        WriteSignal<bool>,
    ),

    /// The timeout duration in milliseconds before the alert auto-closes.
    ///
    /// Defines how long the alert stays visible before automatically closing.
    /// Defaults to `2500` milliseconds (2.5 seconds).
    #[prop(default = 2500)]
    timeout: u64,

    /// The title text for the alert.
    ///
    /// This defines the heading or title of the alert. Defaults to `"Info"`.
    #[prop(default = "Info")]
    title: &'static str,

    /// Text for the confirm button.
    ///
    /// This defines the label for the confirm button within the alert.
    /// Defaults to `"Okay"`.
    #[prop(default = "Okay")]
    confirm_button_text: &'static str,

    /// Text for the cancel button.
    ///
    /// This defines the label for the cancel button within the alert.
    /// Defaults to `"Cancel"`.
    #[prop(default = "Cancel")]
    cancel_button_text: &'static str,

    /// Whether to show the confirm button.
    ///
    /// If `true`, the confirm button is displayed. Defaults to `true`.
    #[prop(default = true)]
    show_confirm_button: bool,

    /// Whether to show the cancel button.
    ///
    /// If `true`, the cancel button is displayed. Defaults to `true`.
    #[prop(default = true)]
    show_cancel_button: bool,

    /// Whether to show the close button.
    ///
    /// If `true`, a close button is included. Defaults to `false`.
    #[prop(default = false)]
    show_close_button: bool,

    /// The position of the alert on the screen.
    ///
    /// Defines where the alert will appear on the screen (e.g., top-left, top-right, bottom-left, bottom-right).
    /// Defaults to `Position::TopRight`.
    #[prop(default = Position::TopRight)]
    position: Position,

    /// The type of icon to display in the alert.
    ///
    /// Defines the icon associated with the alert type (e.g., success, error, warning, info).
    /// Defaults to `IconType::Info`.
    #[prop(default = IconType::Info)]
    icon_type: IconType,

    /// The color of the icon.
    ///
    /// Defines the color of the icon displayed in the alert. This value is applied as a CSS class.
    /// Defaults to an empty string.
    #[prop(default = "")]
    icon_color: &'static str,

    /// The width of the icon.
    ///
    /// Defines the size of the icon in percentage or pixel values. Defaults to `"50"`.
    #[prop(default = "50")]
    icon_width: &'static str,

    /// Custom CSS class for styling the alert container.
    ///
    /// This allows for the styling of the entire alert container, including background color, padding, etc.
    /// Defaults to an empty string.
    #[prop(default = "")]
    alert_class: &'static str,

    /// Custom CSS class for styling the icon in the alert.
    ///
    /// This applies additional styling to the icon inside the alert.
    /// Defaults to an empty string.
    #[prop(default = "")]
    icon_class: &'static str,

    /// Custom CSS class for styling the confirm button.
    ///
    /// This applies additional styling to the confirm button within the alert.
    /// Defaults to an empty string.
    #[prop(default = "")]
    confirm_button_class: &'static str,

    /// Custom CSS class for styling the cancel button.
    ///
    /// This applies additional styling to the cancel button within the alert.
    /// Defaults to an empty string.
    #[prop(default = "")]
    cancel_button_class: &'static str,

    /// Custom CSS class for styling the title of the alert.
    ///
    /// This applies additional styling to the title text within the alert.
    /// Defaults to an empty string.
    #[prop(default = "")]
    title_class: &'static str,

    /// Custom CSS class for styling the message in the alert.
    ///
    /// This applies additional styling to the body message inside the alert.
    /// Defaults to an empty string.
    #[prop(default = "")]
    body_class: &'static str,

    /// Default style for the alert component.
    ///
    /// This defines default inline styles for the alert.
    /// Defaults to `DEFAULT_ALERT_STYLE`.
    #[prop(default = DEFAULT_ALERT_STYLE)]
    alert_style: &'static str,

    /// Default style for the close button.
    ///
    /// This defines the default inline styles for the close button within the alert.
    /// Defaults to `DEFAULT_CLOSE_BUTTON_STYLE`.
    #[prop(default = DEFAULT_CLOSE_BUTTON_STYLE)]
    close_button_style: &'static str,

    /// Default style for the confirm button.
    ///
    /// This defines the default inline styles for the confirm button within the alert.
    /// Defaults to `DEFAULT_CONFIRM_BUTTON_STYLE`.
    #[prop(default = DEFAULT_CONFIRM_BUTTON_STYLE)]
    confirm_button_style: &'static str,

    /// Default style for the cancel button.
    ///
    /// This defines the default inline styles for the cancel button within the alert.
    /// Defaults to `DEFAULT_CANCEL_BUTTON_STYLE`.
    #[prop(default = DEFAULT_CANCEL_BUTTON_STYLE)]
    cancel_button_style: &'static str,

    /// Default style for the icon in the alert.
    ///
    /// This defines the default inline styles for the icon.
    /// Defaults to `DEFAULT_ICON_STYLE`.
    #[prop(default = DEFAULT_ICON_STYLE)]
    icon_style: &'static str,

    /// Default style for the title text in the alert.
    ///
    /// This defines the default inline styles for the title.
    /// Defaults to `DEFAULT_TITLE_STYLE`.
    #[prop(default = DEFAULT_TITLE_STYLE)]
    title_style: &'static str,

    /// Default style for the separator between the title and message.
    ///
    /// This defines the default inline styles for the separator line.
    /// Defaults to `DEFAULT_SEPARATOR_STYLE`.
    #[prop(default = DEFAULT_SEPARATOR_STYLE)]
    separator_style: &'static str,

    /// Default style for the message text in the alert.
    ///
    /// This defines the default inline styles for the message text.
    /// Defaults to `DEFAULT_MESSAGE_STYLE`.
    #[prop(default = DEFAULT_MESSAGE_STYLE)]
    message_style: &'static str,

    /// Whether to use the native browser alert implementation.
    ///
    /// If `true`, it will use the native alert implementation instead of the custom alert component.
    /// Defaults to `false`.
    #[prop(default = false)]
    native: bool,

    /// Callback triggered when the alert opens.
    ///
    /// This is triggered before the alert is shown to the user. Defaults to no-op.
    #[prop(default = Callback::from(move || {}))]
    will_open: Callback<()>,

    /// Callback triggered when the alert has fully opened.
    ///
    /// This is triggered after the alert is shown. Defaults to no-op.
    #[prop(default = Callback::from(move || {}))]
    did_open: Callback<()>,

    /// Callback triggered when the alert closes.
    ///
    /// This is triggered after the alert is closed. Defaults to no-op.
    #[prop(default = Callback::from(move || {}))]
    did_close: Callback<()>,

    /// Callback triggered when the confirm button is clicked.
    ///
    /// This is triggered when the user clicks the confirm button. Defaults to no-op.
    #[prop(default = Callback::from(move || {}))]
    on_confirm: Callback<()>,

    /// Callback triggered when the close button is clicked.
    ///
    /// This is triggered when the user clicks the close button. Defaults to no-op.
    #[prop(default = Callback::from(move || {}))]
    on_close: Callback<()>,

    /// Callback triggered when the cancel button is clicked.
    ///
    /// This is triggered when the user clicks the cancel button. Defaults to no-op.
    #[prop(default = Callback::from(move || {}))]
    on_cancel: Callback<()>,
) -> impl IntoView {
    let delay = Duration::from_millis(timeout);
    Effect::new(move || {
        if show_alert.0.get() && !native {
            will_open.run(());
            set_timeout(
                move || {
                    show_alert.1.set(false);
                    did_close.run(());
                },
                delay,
            );

            did_open.run(());
        } else if show_alert.0.get() && native {
            if let Some(win) = window() {
                will_open.run(());

                let full_message = if !title.is_empty() {
                    format!("{}\n\n{}", title, body)
                } else {
                    body.to_string()
                };

                match (show_confirm_button, show_cancel_button, show_close_button) {
                    (true, true, true) => {
                        if win.confirm_with_message(&full_message).unwrap_or(false) {
                            on_confirm.run(());
                        } else {
                            on_close.run(());
                            on_cancel.run(());
                        }
                    }
                    (true, true, false) => {
                        if win.confirm_with_message(&full_message).unwrap_or(false) {
                            on_confirm.run(());
                        } else {
                            on_cancel.run(());
                        }
                    }
                    (true, false, false) => {
                        win.alert_with_message(&full_message).ok();
                        on_confirm.run(());
                    }
                    _ => {}
                }

                if timeout > 0 {
                    set_timeout(
                        move || {
                            show_alert.1.set(false);
                            did_close.run(());
                        },
                        delay,
                    );
                } else {
                    show_alert.1.set(false);
                }

                did_open.run(());
            }
        }
    });

    let on_cancel = {
        move |_: MouseEvent| {
            on_cancel.run(());
            show_alert.1.set(false);
        }
    };

    let on_confirm = {
        move |_: MouseEvent| {
            on_confirm.run(());
        }
    };

    let position_style = match position {
        Position::TopLeft => "top: 0; left: 0;".to_string(),
        Position::TopCenter => "top: 0; left: 50%; transform: translateX(-50%);".to_string(),
        Position::TopRight => "top: 0; right: 0;".to_string(),
        Position::LeftCenter => "top: 50%; left: 0; transform: translateY(-50%);".to_string(),
        Position::Center => "top: 50%; left: 50%; transform: translate(-50%, -50%);".to_string(),
        Position::BottomCenter => "bottom: 0; left: 50%; transform: translateX(-50%);".to_string(),
        Position::RightCenter => "top: 50%; right: 0; transform: translateY(-50%);".to_string(),
        Position::BottomRight => "bottom: 0; right: 0;".to_string(),
        Position::BottomLeft => "bottom: 0; left: 0;".to_string(),
        Position::Custom(x, y) => format!("top: {}; left: {};", y, x),
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
        icon_color
    };

    view! {
        {move || {
            // SVGs taken from: https://fontawesome.com/icons
            let icon_tag = match icon_type {
                IconType::Warning => view! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={icon_width}
                        style={icon_style}
                        class="p-2 m-2"
                        fill={icon_color}
                        viewBox="0 0 512 512"
                    >
                        <path
                            d="M248.4 84.3c1.6-2.7 4.5-4.3 7.6-4.3s6 1.6 7.6 4.3L461.9 410c1.4 2.3 2.1 4.9 2.1 7.5c0 8-6.5 14.5-14.5 14.5H62.5c-8 0-14.5-6.5-14.5-14.5c0-2.7 .7-5.3 2.1-7.5L248.4 84.3zm-41-25L9.1 385c-6 9.8-9.1 21-9.1 32.5C0 452 28 480 62.5 480h387c34.5 0 62.5-28 62.5-62.5c0-11.5-3.2-22.7-9.1-32.5L304.6 59.3C294.3 42.4 275.9 32 256 32s-38.3 10.4-48.6 27.3zM288 368a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm-8-184c0-13.3-10.7-24-24-24s-24 10.7-24 24v96c0 13.3 10.7 24 24 24s24-10.7 24-24V184z"
                        />
                    </svg>
                }.into_any(),
                IconType::Error => view! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={icon_width}
                        style={icon_style}
                        class="p-2 m-2"
                        fill={icon_color}
                        viewBox="0 0 20 20"
                    >
                        <path
                            d="M12.71,7.291c-0.15-0.15-0.393-0.15-0.542,0L10,9.458L7.833,7.291c-0.15-0.15-0.392-0.15-0.542,0c-0.149,0.149-0.149,0.392,0,0.541L9.458,10l-2.168,2.167c-0.149,0.15-0.149,0.393,0,0.542c0.15,0.149,0.392,0.149,0.542,0L10,10.542l2.168,2.167c0.149,0.149,0.392,0.149,0.542,0c0.148-0.149,0.148-0.392,0-0.542L10.542,10l2.168-2.168C12.858,7.683,12.858,7.44,12.71,7.291z M10,1.188c-4.867,0-8.812,3.946-8.812,8.812c0,4.867,3.945,8.812,8.812,8.812s8.812-3.945,8.812-8.812C18.812,5.133,14.867,1.188,10,1.188z M10,18.046c-4.444,0-8.046-3.603-8.046-8.046c0-4.444,3.603-8.046,8.046-8.046c4.443,0,8.046,3.602,8.046,8.046C18.046,14.443,14.443,18.046,10,18.046z"
                        />
                    </svg>
                }.into_any(),
                IconType::Success => view! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={icon_width}
                        style={icon_style}
                        class="p-2 m-2"
                        fill={icon_color}
                        viewBox="0 0 512 512"
                    >
                        <path
                            d="M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM369 209c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-111 111-47-47c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9l64 64c9.4 9.4 24.6 9.4 33.9 0L369 209z"
                        />
                    </svg>
                }.into_any(),
                IconType::Info => view! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={icon_width}
                        style={icon_style}
                        class="p-2 m-2"
                        fill={icon_color}
                        viewBox="0 0 16 16"
                    >
                        <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z" />
                        <path
                            d="m8.93 6.588-2.29.287-.082.38.45.083c.294.07.352.176.288.469l-.738 3.468c-.194.897.105 1.319.808 1.319.545 0 1.178-.252 1.465-.598l.088-.416c-.2.176-.492.246-.686.246-.275 0-.375-.193-.304-.533L8.93 6.588zM9 4.5a1 1 0 1 1-2 0 1 1 0 0 1 2 0z"
                        />
                    </svg>
                }.into_any(),
                IconType::Question => view! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={icon_width}
                        style={icon_style}
                        class="p-2 m-2"
                        fill={icon_color}
                        viewBox="0 0 16 16"
                    >
                        <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z" />
                        <path
                            d="M5.255 5.786a.237.237 0 0 0 .241.247h.825c.138 0 .248-.113.266-.25.09-.656.54-1.134 1.342-1.134.686 0 1.314.343 1.314 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.003.217a.25.25 0 0 0 .25.246h.811a.25.25 0 0 0 .25-.25v-.105c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.267 0-2.655.59-2.75 2.286zm1.557 5.763c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94z"
                        />
                    </svg>
                }.into_any(),
            };
            if show_alert.0.get() {
                Some(view! {
                    <div style={alert_style}>
                        <div
                            class={alert_class}
                            style=format!("position: absolute; {}", position_style.to_string())
                        >
                            {if show_close_button {
                                Some(view! {
                                    <button style={close_button_style} on:click=on_cancel>{"X"}</button>
                                })
                            } else {
                                None
                            }}
                            <div class={icon_class} style={icon_style}>
                                {icon_tag}
                            </div>
                            <strong style={title_style} class={title_class}>
                                {title}
                            </strong>
                            <hr style={separator_style} />
                            <p class={body_class} style={message_style}>
                                {body}
                            </p>
                            {if show_confirm_button {
                                Some(view! {
                                    <button
                                        class={confirm_button_class}
                                        style={confirm_button_style}
                                        on:click=on_confirm
                                    >
                                        {confirm_button_text}
                                    </button>
                                })
                            } else {
                                None
                            }}
                            {if show_cancel_button {
                                Some(view! {
                                    <button
                                        class={cancel_button_class}
                                        style={cancel_button_style}
                                        on:click=on_cancel
                                    >
                                        {cancel_button_text}
                                    </button>
                                })
                            } else {
                                None
                            }}
                        </div>
                    </div>
                })
            } else {
                None
            }
        }}
    }
}
