## Y Alert RS Yew Usage

Adding Alert RS to your project is simple:

1. Make sure your project is set up with **Yew**. Refer to their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Alert component to your dependencies by including it in your `Cargo.toml` file.

   ```sh
   cargo add alert-rs --features=yew
   ```

1. Import the `Alert` component into your Yew component and start showing alerts in your app.

## 🛠️ Usage

Incorporating Yew Alert into your application is easy. Follow these steps:

1. Import the Alert component into your Yew project:

   ```rust
   use yew::prelude::*;
   use alert_rs::yew::Alert;
   ```

1. Define the alert properties and use the Alert component in your Yew component:

   ```rust
   use yew::prelude::*;
   use alert_rs::yew::Alert;
   use alert_rs::{IconType, Position};


   #[function_component(App)]
   pub fn app() -> Html {
       let show_alert = use_state(|| true);
       html! {
           <Alert
               title={"Alert Title"}
               body={"This is an alert message"}
               show_alert={show_alert}
               timeout={2500}
               alert_class={"w-96 h-48 text-white"}
               icon_class={"flex justify-center"}
               confirm_button_text={"Okay"}
               cancel_button_text={"Cancel"}
               confirm_button_class={"bg-green-500 text-white rounded"}
               cancel_button_class={"bg-red-500 text-white rounded"}
               show_confirm_button={true}
               show_cancel_button={true}
               show_close_button={true}
               on_confirm={Callback::noop()}
               on_cancel={Callback::noop()}
               position={Position::TopRight}
               icon_type={IconType::Success}
               container_class={"flex items-center text-center justify-center bg-gray-800 text-white border border-gray-600"}
               title_class={"dark:text-white"}
               body_class={"dark:text-gray-300"}
               icon_color={""}
               icon_width={"50"}
           />
       }
   }
   ```

## 🔧 Props

### Main Props

| Property              | Type                   | Description                                                   | Default   |
| --------------------- | ---------------------- | ------------------------------------------------------------- | --------- |
| `show_alert`          | `UseStateHandle<bool>` | The state handle controlling the visibility of the alert.     | `false`   |
| `title`               | `&'static str`         | The title text for the alert.                                 | `"Info"`  |
| `body`                | `&'static str`         | The message content of the alert.                             | `""`      |
| `timeout`             | `u32`                  | Timeout duration in milliseconds for the alert to auto-close. | `2500` ms |
| `show_confirm_button` | `bool`                 | Whether to display the confirm button.                        | `true`    |
| `show_cancel_button`  | `bool`                 | Whether to display the cancel button.                         | `true`    |
| `show_close_button`   | `bool`                 | Whether to display the close button.                          | `false`   |

### Callback Props

| Property     | Type           | Description                                            | Default |
| ------------ | -------------- | ------------------------------------------------------ | ------- |
| `on_confirm` | `Callback<()>` | Callback triggered when the confirm button is clicked. | No-op   |
| `on_cancel`  | `Callback<()>` | Callback triggered when the cancel button is clicked.  | No-op   |
| `on_close`   | `Callback<()>` | Callback triggered when the close button is clicked.   | No-op   |
| `will_open`  | `Callback<()>` | Callback triggered before the alert opens.             | No-op   |
| `did_open`   | `Callback<()>` | Callback triggered after the alert opens.              | No-op   |
| `did_close`  | `Callback<()>` | Callback triggered after the alert closes.             | No-op   |

### Alert Appearance & Positioning

| Property     | Type           | Description                                                           | Default          |
| ------------ | -------------- | --------------------------------------------------------------------- | ---------------- |
| `native`     | `bool`         | Whether to use the native browser alert instead of custom one.        | `false`          |
| `position`   | `Position`     | Position of the alert on the screen (`Position::TopRight`, etc.).     | `TopRight`       |
| `icon_type`  | `IconType`     | The type of icon to display with the alert (e.g., `Info`, `Warning`). | `IconType::Info` |
| `icon_color` | `&'static str` | The color of the icon.                                                | `""`             |
| `icon_width` | `&'static str` | The width of the icon.                                                | `"50"`           |

### Styling Props

```sh
+-----------------------------------------------------------+  <-- `alert_class`
|                                                           |
|  +-----------------------------------------------+        |  <-- `close_button_style` (if `show_close_button`)
|  |               [X] Close Button                |        |
|  +-----------------------------------------------+        |
|                                                           |
|  +-----------------------------------------------+        |  <-- `icon_class` and `icon_style`
|  |                  [Icon]                       |        |  <-- `icon_tag`
|  +-----------------------------------------------+        |
|                                                           |
|  +-----------------------------------------------+        |  <-- `title_class` and `title_style`
|  |                [Alert Title]                  |        |  <-- `props.title`
|  +-----------------------------------------------+        |
|                                                           |
|  +-----------------------------------------------+        |  <-- `separator_style`
|  |             [--- Separator ---]               |        |
|  +-----------------------------------------------+        |
|                                                           |
|  +-----------------------------------------------+        |  <-- `message_style` and `body_class`
|  |                [Alert Message]                |        |  <-- `props.body`
|  +-----------------------------------------------+        |
|                                                           |
|  +-----------------------------------------------+        |  <-- `confirm_button_class` and `confirm_button_style`
|  |                [Confirm Button]               |        |  <-- `props.confirm_button_text`
|  +-----------------------------------------------+        |
|                                                           |
|  +-----------------------------------------------+        |  <-- `cancel_button_class` and `cancel_button_style`
|  |                [Cancel Button]                |        |  <-- `props.cancel_button_text`
|  +-----------------------------------------------+        |
|                                                           |
+-----------------------------------------------------------+
```

| Property               | Type           | Description                                          | Default |
| ---------------------- | -------------- | ---------------------------------------------------- | ------- |
| `alert_class`          | `&'static str` | CSS class for styling the alert container.           | `""`    |
| `icon_class`           | `&'static str` | CSS class for styling the icon.                      | `""`    |
| `confirm_button_class` | `&'static str` | CSS class for styling the confirm button.            | `""`    |
| `cancel_button_class`  | `&'static str` | CSS class for styling the cancel button.             | `""`    |
| `container_class`      | `&'static str` | CSS class for styling the alert container.           | `""`    |
| `title_class`          | `&'static str` | CSS class for styling the alert title.               | `""`    |
| `message_class`        | `&'static str` | CSS class for styling the message text in the alert. | `""`    |

### Inline Styles

| Property               | Type           | Description                               | Default                        |
| ---------------------- | -------------- | ----------------------------------------- | ------------------------------ |
| `alert_style`          | `&'static str` | Inline CSS styles for the alert.          | `DEFAULT_ALERT_STYLE`          |
| `close_button_style`   | `&'static str` | Inline CSS styles for the close button.   | `DEFAULT_CLOSE_BUTTON_STYLE`   |
| `confirm_button_style` | `&'static str` | Inline CSS styles for the confirm button. | `DEFAULT_CONFIRM_BUTTON_STYLE` |
| `cancel_button_style`  | `&'static str` | Inline CSS styles for the cancel button.  | `DEFAULT_CANCEL_BUTTON_STYLE`  |
| `icon_style`           | `&'static str` | Inline CSS styles for the icon.           | `DEFAULT_ICON_STYLE`           |
| `title_style`          | `&'static str` | Inline CSS styles for the title text.     | `DEFAULT_TITLE_STYLE`          |
| `separator_style`      | `&'static str` | Inline CSS styles for the separator.      | `DEFAULT_SEPARATOR_STYLE`      |
| `message_style`        | `&'static str` | Inline CSS styles for the message text.   | `DEFAULT_MESSAGE_STYLE`        |

## 💡 Notes

- The `native` prop can be set to `true` to use the browser's default alert behavior instead of the custom component.
- The alert is displayed based on the `show_alert` state, which should be controlled by the parent component.
- Timeout behavior can be adjusted using the `timeout` property, and alert visibility can be toggled using the `show_alert` state.
- You can customize the alert's appearance, including the icon, buttons, position, and styles.
