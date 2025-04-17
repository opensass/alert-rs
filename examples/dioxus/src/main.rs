use alert_rs::dioxus::Alert;
use alert_rs::{IconType, Position};
use dioxus::prelude::*;
use dioxus_logger::tracing;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("main.css") }
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        Home {}
    }
}

#[component]
pub fn Home() -> Element {
    let mut show_alert_0 = use_signal(|| false);
    let mut show_alert_1 = use_signal(|| false);
    let mut show_alert_2 = use_signal(|| false);
    let mut show_alert_3 = use_signal(|| false);
    let mut show_alert_4 = use_signal(|| false);
    let mut show_alert_5 = use_signal(|| false);
    let mut show_alert_6 = use_signal(|| false);
    let mut show_alert_7 = use_signal(|| false);
    let mut show_alert_8 = use_signal(|| false);
    let mut show_alert_9 = use_signal(|| false);
    let mut show_alert_10 = use_signal(|| false);
    let mut show_alert_11 = use_signal(|| false);
    let mut show_alert_12 = use_signal(|| false);

    rsx! {
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 {
                class: "text-3xl font-bold mb-8 text-white",
                "Alert RS Dioxus Examples"
            }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8",

                // Native Blocking Alert
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Native Blocking Alert" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto",
                        r#"Alert \{{
    title: "Native Alert",
    body: "This is a native blocking alert that exists in your browser.",
    show_alert: show_alert_state,
    native: true,
\}}"#
                    }
                    button {
                        class: "mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| show_alert_0.set(true),
                        "Show Alert"
                    }
                    Alert {
                        title: "Native Alert",
                        body: "This is a native blocking alert that exists in your browser.",
                        show_alert: show_alert_0,
                        native: true,
                    }
                }

                // Top Right - Error With Confirmation
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Top Right - Error With Confirmation" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto",
                        r#"Alert \{{
    title: "Error Alert",
    body: "This is an error alert in the top-right corner.",
    show_alert: show_alert_state,
    position: Position::TopRight,
    icon_type: IconType::Error,
    alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
    title_class: "text-sm font-semibold",
    body_class: "text-xs",
    on_confirm: move |_| {{ show_second_alert.set(true); }},
\}}"#
                    }
                    button {
                        class: "mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| show_alert_1.set(true),
                        "Show Alert"
                    }
                    Alert {
                        title: "Error Alert",
                        body: "This is an error alert in the top-right corner.",
                        show_alert: show_alert_1,
                        position: Position::TopRight,
                        icon_type: IconType::Error,
                        alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
                        title_class: "text-sm font-semibold",
                        body_class: "text-xs",
                        on_confirm: move |_| show_alert_12.set(true),
                    }
                    Alert {
                        title: "Confirm Alert",
                        body: "Are you sure?",
                        show_alert: show_alert_12,
                        position: Position::TopRight,
                        icon_type: IconType::Error,
                        alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
                        title_class: "text-sm font-semibold",
                        body_class: "text-xs",
                    }
                }

                // Bottom Left - Success
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Bottom Left - Success" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto",
                        r#"Alert \{{
    title: "Success Alert",
    body: "This is a success alert in the bottom-left corner.",
    show_alert: show_alert_state,
    position: Position::BottomLeft,
    icon_type: IconType::Success,
    alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
    title_class: "text-sm font-semibold",
    body_class: "text-xs",
\}}"#
                    }
                    button {
                        class: "mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| show_alert_2.set(true),
                        "Show Alert"
                    }
                    Alert {
                        title: "Success Alert",
                        body: "This is a success alert in the bottom-left corner.",
                        show_alert: show_alert_2,
                        position: Position::BottomLeft,
                        icon_type: IconType::Success,
                        alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
                        title_class: "text-sm font-semibold",
                        body_class: "text-xs",
                    }
                }

                // Bottom Right - Info
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Bottom Right - Info" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto",
                        r#"Alert \{{
    title: "Info Alert",
    body: "This is an info alert in the bottom-right corner.",
    show_alert: show_alert_state,
    position: Position::BottomRight,
    icon_type: IconType::Info,
    alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
    title_class: "text-sm font-semibold",
    body_class: "text-xs",
\}}"#
                    }
                    button {
                        class: "mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| show_alert_3.set(true),
                        "Show Alert"
                    }
                    Alert {
                        title: "Info Alert",
                        body: "This is an info alert in the bottom-right corner.",
                        show_alert: show_alert_3,
                        position: Position::BottomRight,
                        icon_type: IconType::Info,
                        alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
                        title_class: "text-sm font-semibold",
                        body_class: "text-xs",
                    }
                }

                // Bottom Left - Warning
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Bottom Left - Warning" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto",
                        r#"Alert \{{
    title: "Warning Alert",
    body: "This is a warning alert in the bottom-left corner.",
    show_alert: show_alert_7,
    position: Position::BottomLeft,
    icon_type: IconType::Warning,
    alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
    title_class: "text-sm font-semibold",
    body_class: "text-xs",
\}}"#
                    }
                    button {
                        class: "mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| show_alert_7.set(true),
                        "Show Alert"
                    }
                    Alert {
                        title: "Warning Alert",
                        body: "This is a warning alert in the bottom-left corner.",
                        show_alert: show_alert_7,
                        position: Position::BottomLeft,
                        icon_type: IconType::Warning,
                        alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
                        title_class: "text-sm font-semibold",
                        body_class: "text-xs",
                    }
                }

                // Centered - Success
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Centered - Success" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto",
                        r#"Alert \{{
    title: "Centered Success Alert",
    body: "This is a centered success alert.",
    show_alert: show_alert_8,
    position: Position::Center,
    icon_type: IconType::Success,
    alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
    title_class: "text-sm font-semibold",
    body_class: "text-xs",
\}}"#
                    }
                    button {
                        class: "mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| show_alert_8.set(true),
                        "Show Alert"
                    }
                    Alert {
                        title: "Centered Success Alert",
                        body: "This is a centered success alert.",
                        show_alert: show_alert_8,
                        position: Position::Center,
                        icon_type: IconType::Success,
                        alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
                        title_class: "text-sm font-semibold",
                        body_class: "text-xs",
                    }
                }

                // Bottom Right - Warning
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Bottom Right - Warning" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto",
                        r#"Alert \{{
    title: "Warning Alert",
    body: "This is a warning alert in the bottom-right corner.",
    show_alert: show_alert_9,
    position: Position::BottomRight,
    icon_type: IconType::Warning,
    alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
    title_class: "text-sm font-semibold",
    body_class: "text-xs",
\}}"#
                    }
                    button {
                        class: "mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| show_alert_9.set(true),
                        "Show Alert"
                    }
                    Alert {
                        title: "Warning Alert",
                        body: "This is a warning alert in the bottom-right corner.",
                        show_alert: show_alert_9,
                        position: Position::BottomRight,
                        icon_type: IconType::Warning,
                        alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
                        title_class: "text-sm font-semibold",
                        body_class: "text-xs",
                    }
                }

                // Top Center - Error
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Top Center - Error" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto",
                        r#"Alert \{{
    title: "Error Alert",
    body: "This is an error alert in the top-center of the screen.",
    show_alert: show_alert_10,
    position: Position::TopCenter,
    icon_type: IconType::Error,
    alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
    title_class: "text-sm font-semibold",
    body_class: "text-xs",
\}}"#
                    }
                    button {
                        class: "mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| show_alert_10.set(true),
                        "Show Alert"
                    }
                    Alert {
                        title: "Error Alert",
                        body: "This is an error alert in the top-center of the screen.",
                        show_alert: show_alert_10,
                        position: Position::TopCenter,
                        icon_type: IconType::Error,
                        alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
                        title_class: "text-sm font-semibold",
                        body_class: "text-xs",
                    }
                }

                // Top Left - Info
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Top Left - Info" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto",
                        r#"Alert \{{
    title: "Info Alert",
    body: "This is an info alert in the top-left corner.",
    show_alert: show_alert_11,
    position: Position::TopLeft,
    icon_type: IconType::Info,
    alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
    title_class: "text-sm font-semibold",
    body_class: "text-xs",
\}}"#
                    }
                    button {
                        class: "mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| show_alert_11.set(true),
                        "Show Alert"
                    }
                    Alert {
                        title: "Info Alert",
                        body: "This is an info alert in the top-left corner.",
                        show_alert: show_alert_11,
                        position: Position::TopLeft,
                        icon_type: IconType::Info,
                        alert_class: "text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4",
                        title_class: "text-sm font-semibold",
                        body_class: "text-xs",
                    }
                }
            }
        }
    }
}