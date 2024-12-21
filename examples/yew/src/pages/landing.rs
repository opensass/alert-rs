use alert_rs::yew::Alert;
use alert_rs::{IconType, Position};
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let show_alert_0 = use_state(|| false);
    let show_alert_1 = use_state(|| false);
    let show_alert_2 = use_state(|| false);
    let show_alert_3 = use_state(|| false);
    let show_alert_4 = use_state(|| false);
    let show_alert_5 = use_state(|| false);
    let show_alert_6 = use_state(|| false);
    let show_alert_7 = use_state(|| false);
    let show_alert_8 = use_state(|| false);
    let show_alert_9 = use_state(|| false);
    let show_alert_10 = use_state(|| false);
    let show_alert_11 = use_state(|| false);
    let show_alert_12 = use_state(|| false);

    let callback_0 = {
        let show_alert_0 = show_alert_0.clone();
        Callback::from(move |_| {
            show_alert_0.set(true);
        })
    };
    let callback_1 = {
        let show_alert_1 = show_alert_1.clone();
        Callback::from(move |_| {
            show_alert_1.set(true);
        })
    };
    let callback_2 = {
        let show_alert_2 = show_alert_2.clone();
        Callback::from(move |_| {
            show_alert_2.set(true);
        })
    };
    let callback_3 = {
        let show_alert_3 = show_alert_3.clone();
        Callback::from(move |_| {
            show_alert_3.set(true);
        })
    };
    let callback_4 = {
        let show_alert_4 = show_alert_4.clone();
        Callback::from(move |_| {
            show_alert_4.set(true);
        })
    };
    let callback_5 = {
        let show_alert_5 = show_alert_5.clone();
        Callback::from(move |_| {
            show_alert_5.set(true);
        })
    };
    let callback_6 = {
        let show_alert_6 = show_alert_6.clone();
        Callback::from(move |_| {
            show_alert_6.set(true);
        })
    };
    let callback_7 = {
        let show_alert_7 = show_alert_7.clone();
        Callback::from(move |_| {
            show_alert_7.set(true);
        })
    };
    let callback_8 = {
        let show_alert_8 = show_alert_8.clone();
        Callback::from(move |_| {
            show_alert_8.set(true);
        })
    };
    let callback_9 = {
        let show_alert_9 = show_alert_9.clone();
        Callback::from(move |_| {
            show_alert_9.set(true);
        })
    };
    let callback_10 = {
        let show_alert_10 = show_alert_10.clone();
        Callback::from(move |_| {
            show_alert_10.set(true);
        })
    };
    let callback_11 = {
        let show_alert_11 = show_alert_11.clone();
        Callback::from(move |_| {
            show_alert_11.set(true);
        })
    };
    let callback_12 = {
        let show_alert_12 = show_alert_12.clone();
        Callback::from(move |_| {
            show_alert_12.set(true);
        })
    };

    html! {
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
                        onclick={callback_0}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Native Alert"
                        body="This is a native blocking alert that exists in your browser."
                        show_alert={show_alert_0.clone()}
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
    on_confirm={Callback::from(move |_| { show_second_alert.set(true); })}
/>"# }
                    </pre>
                    <button
                        class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        onclick={callback_1}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Error Alert"
                        body="This is an error alert in the top-right corner."
                        show_alert={show_alert_1.clone()}
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
                        show_alert={show_alert_12.clone()}
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
                        onclick={callback_2}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Success Alert"
                        body="This is a success alert in the bottom-left corner."
                        show_alert={show_alert_2.clone()}
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
                        onclick={callback_3}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Info Alert"
                        body="This is an info alert in the bottom-right corner."
                        show_alert={show_alert_3.clone()}
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
                        onclick={callback_4}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Error Alert"
                        body="This is an error alert in the top-left corner."
                        show_alert={show_alert_4.clone()}
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
                        onclick={callback_5}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Success Alert"
                        body="This is a success alert in the top-right corner."
                        show_alert={show_alert_5.clone()}
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
                        onclick={callback_6}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Info Alert"
                        body="This is an info alert in the bottom-left corner."
                        show_alert={show_alert_6.clone()}
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
                        onclick={callback_7}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Warning Alert"
                        body="This is a warning alert in the bottom-right corner."
                        show_alert={show_alert_7.clone()}
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
                        onclick={callback_8}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Info Alert"
                        body="This is an info alert in the top-center."
                        show_alert={show_alert_8.clone()}
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
                        onclick={callback_9}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Success Alert"
                        body="This is a success alert in the top-left corner."
                        show_alert={show_alert_9.clone()}
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
                        onclick={callback_10}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Warning Alert"
                        body="This is a warning alert in the bottom-center."
                        show_alert={show_alert_10.clone()}
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
                        onclick={callback_11}
                    >
                        { "Show Alert" }
                    </button>
                    <Alert
                        title="Success Alert"
                        body="This is a success alert in the bottom-right corner."
                        show_alert={show_alert_11.clone()}
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
