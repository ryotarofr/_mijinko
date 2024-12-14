// delete forever

use crate::context::theme_context::Theme;
use dioxus::prelude::*;
use keyboard_types::{Code, Modifiers};

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut theme = use_context::<Signal<Theme>>();

    let handle_change = move |event: Event<FormData>| {
        let selected_value = event.data.value().clone();
        let new_theme = match selected_value.as_str() {
            "Default" => Theme::Default,
            "Readonly" => Theme::Readonly,
            "Dev" => Theme::Dev,
            _ => Theme::Default,
        };
        theme.set(new_theme);
    };

    let handle_keydown = move |event: Event<KeyboardData>| {
        if event.modifiers().contains(Modifiers::META) && event.code() == Code::KeyH {
            let new_theme = match *theme.read() {
                Theme::Default => Theme::Readonly,
                Theme::Readonly => Theme::Dev,
                Theme::Dev => Theme::Default,
            };
            theme.set(new_theme);
            event.stop_propagation();
        }
    };

    rsx! {
        div {
            // label { "Choose Theme: " }
            onkeydown: handle_keydown,
            select { onchange: handle_change,
                option { value: "Default", "Default" }
                option { value: "Readonly", "Readonly" }
                option { value: "Dev", "Dev" }
            }
        }
    }
}
