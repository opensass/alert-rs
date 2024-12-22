use leptos::prelude::*;
use alert_rs::leptos::Alert;
use alert_rs::{IconType, Position};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Home />
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let show_alert_0 = signal(false);
    let show_alert_1 = signal(false);
    let show_alert_2 = signal(false);
    let show_alert_3 = signal(false);
    let show_alert_4 = signal(false);
    let show_alert_5 = signal(false);
    let show_alert_6 = signal(false);
    let show_alert_7 = signal(false);
    let show_alert_8 = signal(false);
    let show_alert_9 = signal(false);
    let show_alert_10 = signal(false);
    let show_alert_11 = signal(false);
    let show_alert_12 = signal(false);

    let callback_0 = {
        move |_| {
            show_alert_0.1.set(true);
        }
    };
    let callback_1 = {
        move |_| {
            show_alert_1.1.set(true);
        }
    };
    let callback_2 = {
        move |_| {
            show_alert_2.1.set(true);
        }
    };
    let callback_3 = {
        move |_| {
            show_alert_3.1.set(true);
        }
    };
    let callback_4 = {
        move |_| {
            show_alert_4.1.set(true);
        }
    };
    let callback_5 = {
        move |_| {
            show_alert_5.1.set(true);
        }
    };
    let callback_6 = {
        move |_| {
            show_alert_6.1.set(true);
        }
    };
    let callback_7 = {
        move |_| {
            show_alert_7.1.set(true);
        }
    };
    let callback_8 = {
        move |_| {
            show_alert_8.1.set(true);
        }
    };
    let callback_9 = {
        move |_| {
            show_alert_9.1.set(true);
        }
    };
    let callback_10 = {
        move |_| {
            show_alert_10.1.set(true);
        }
    };
    let callback_11 = {
        move |_| {
            show_alert_11.1.set(true);
        }
    };
    let callback_12 = {
        Callback::from(move || {
            show_alert_12.1.set(true);
        })
    };

    view! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Alert RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8">
                // Native Blocking Alert
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Native Blocking Alert" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Native Alert"
    body="This is a native blocking alert that exists in your browser."
    show_alert={show_alert_state}
    native=true
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_0}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Native Alert"
                        body="This is a native blocking alert that exists in your browser."
                        show_alert={show_alert_0}
                        native=true
                    />
                </div>
                // Top Right - Error With Confirmation
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">
                        { "Top Right - Error With Confirmation" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Error Alert"
    body="This is an error alert in the top-right corner."
    show_alert={show_alert_state}
    position={Position::TopRight}
    icon_type={IconType::Error}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
    on_confirm={move |_| { show_second_alert.1.set(true); }}
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_1}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Error Alert"
                        body="This is an error alert in the top-right corner."
                        show_alert={show_alert_1}
                        position={Position::TopRight}
                        icon_type={IconType::Error}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                        on_confirm={callback_12}
                    />
                    <Alert
                        title="Confirm Alert"
                        body="Are you sure?"
                        show_alert={show_alert_12}
                        position={Position::TopRight}
                        icon_type={IconType::Error}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Bottom Left - Success
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Bottom Left - Success" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Success Alert"
    body="This is a success alert in the bottom-left corner."
    show_alert={show_alert_state}
    position={Position::BottomLeft}
    icon_type={IconType::Success}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_2}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Success Alert"
                        body="This is a success alert in the bottom-left corner."
                        show_alert={show_alert_2}
                        position={Position::BottomLeft}
                        icon_type={IconType::Success}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Bottom Right - Info
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Bottom Right - Info" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Info Alert"
    body="This is an info alert in the bottom-right corner."
    show_alert={show_alert_state}
    position={Position::BottomRight}
    icon_type={IconType::Info}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_3}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Info Alert"
                        body="This is an info alert in the bottom-right corner."
                        show_alert={show_alert_3}
                        position={Position::BottomRight}
                        icon_type={IconType::Info}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Top Left - Error
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Top Left - Error" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Error Alert"
    body="This is an error alert in the top-left corner."
    show_alert={show_alert_state}
    position={Position::TopLeft}
    icon_type={IconType::Error}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_4}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Error Alert"
                        body="This is an error alert in the top-left corner."
                        show_alert={show_alert_4}
                        position={Position::TopLeft}
                        icon_type={IconType::Error}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Top Right - Success
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Top Right - Success" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Success Alert"
    body="This is a success alert in the top-right corner."
    show_alert={show_alert_state}
    position={Position::TopRight}
    icon_type={IconType::Success}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_5}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Success Alert"
                        body="This is a success alert in the top-right corner."
                        show_alert={show_alert_5}
                        position={Position::TopRight}
                        icon_type={IconType::Success}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Left Center - Info
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Left Center - Info" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Info Alert"
    body="This is an info alert in the bottom-left corner."
    show_alert={show_alert_state}
    position={Position::LeftCenter}
    icon_type={IconType::Info}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_6}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Info Alert"
                        body="This is an info alert in the bottom-left corner."
                        show_alert={show_alert_6}
                        position={Position::LeftCenter}
                        icon_type={IconType::Info}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Right Center - Warning
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Right Center - Warning" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Warning Alert"
    body="This is a warning alert in the bottom-right corner."
    show_alert={show_alert_state}
    position={Position::RightCenter}
    icon_type={IconType::Warning}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_7}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Warning Alert"
                        body="This is a warning alert in the bottom-right corner."
                        show_alert={show_alert_7}
                        position={Position::RightCenter}
                        icon_type={IconType::Warning}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Top Center - Info
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Top Center - Info" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Info Alert"
    body="This is an info alert in the top-center."
    show_alert={show_alert_state}
    position={Position::TopCenter}
    icon_type={IconType::Info}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_8}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Info Alert"
                        body="This is an info alert in the top-center."
                        show_alert={show_alert_8}
                        position={Position::TopCenter}
                        icon_type={IconType::Info}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Center - Success
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Center - Success" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Success Alert"
    body="This is a success alert in the top-left corner."
    show_alert={show_alert_state}
    position={Position::Center}
    icon_type={IconType::Success}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_9}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Success Alert"
                        body="This is a success alert in the top-left corner."
                        show_alert={show_alert_9}
                        position={Position::Center}
                        icon_type={IconType::Success}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Bottom Center - Warning
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Bottom Center - Warning" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Warning Alert"
    body="This is a warning alert in the bottom-center."
    show_alert={show_alert_state}
    position={Position::BottomCenter}
    icon_type={IconType::Warning}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_10}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Warning Alert"
                        body="This is a warning alert in the bottom-center."
                        show_alert={show_alert_10}
                        position={Position::BottomCenter}
                        icon_type={IconType::Warning}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
                // Bottom Right - Success
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Bottom Right - Success" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Alert
    title="Success Alert"
    body="This is a success alert in the bottom-right corner."
    show_alert={show_alert_state}
    position={Position::BottomRight}
    icon_type={IconType::Success}
    alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
    title_class="text-sm font-semibold"
    body_class="text-xs"
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click={callback_11}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Success Alert"
                        body="This is a success alert in the bottom-right corner."
                        show_alert={show_alert_11}
                        position={Position::BottomRight}
                        icon_type={IconType::Success}
                        alert_class="text-center w-96 h-48 bg-white text-black rounded-md shadow-lg p-4"
                        title_class="text-sm font-semibold"
                        body_class="text-xs"
                    />
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount::mount_to_body(|| view! { <App/> });
}
