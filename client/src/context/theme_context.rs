use dioxus::prelude::*;
use once_cell::sync::Lazy;
use std::{rc::Rc, sync::Arc};
use wasm_bindgen::JsCast;
use web_sys::{window, Document};

use crate::r#fn::get_context_default_value_factory::get_context_default_value_factory;

#[derive(Clone, Copy, Debug)]
pub enum Theme {
    Default,
    Readonly,
    Dev,
}

#[derive(Clone, Copy)]
struct Hsl {
    hue: f32,
    saturation: f32,
    lightness: f32,
}

impl Hsl {
    fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Self {
            hue: h,
            saturation: s,
            lightness: l,
        }
    }
}

#[derive(Clone)]
struct ColorScheme {
    main: Hsl,
    background: Hsl,
    emphasis: Hsl,
    emphasis_main: Hsl,
    warn: Hsl,
    warn_main: Hsl,
    light: Hsl,
    dark: Hsl,
    mode: Hsl,
}

#[derive(Clone)]
pub struct ThemeContextData {
    colors: ColorScheme,
    is_dark_mode: bool,
    current_theme: Rc<dyn Fn(Theme) -> ColorScheme>,
    pub set_theme: Theme,
}

#[derive(Clone, Props, PartialEq)]
pub struct ThemeProviderProps {
    children: Element,
}

static NO_IMPLE: Lazy<Arc<dyn Fn() + Send + Sync>> = Lazy::new(|| {
    let factory = get_context_default_value_factory("ThemeProvider");
    factory("default_member_name")
});

/// Adding a style tag to the `head` tag and setting a CSS variable provides context for changing the theme.
/// Theme:
///  - Default
///  - Readonly
///  - Dev
///
/// Mode:
///  - light
///  - dark
pub fn ThemeProvider(cx: ThemeProviderProps) -> Element {
    NO_IMPLE();
    use_context_provider(|| Signal::new(Theme::Default));
    let theme = use_context::<Signal<Theme>>();
    use_context_provider(|| Signal::new(false));
    let is_dark_mode = use_context::<Signal<bool>>();

    let color_scheme = generate_color_scheme(*theme.read(), *is_dark_mode.read());

    let theme_context_data = ThemeContextData {
        colors: color_scheme.clone(),
        is_dark_mode: *is_dark_mode.read(),
        current_theme: Rc::new(move |theme| generate_color_scheme(theme, *is_dark_mode.read())),
        set_theme: *theme.read(),
    };

    let style_content = format! {
      ":root {{ {} {} {} {} {} {} {} {} {} --color-scheme: {}; }}",
            get_color_css_variables("main", color_scheme.main),
            get_color_css_variables("bg", color_scheme.background),
            get_color_css_variables("em", color_scheme.emphasis),
            get_color_css_variables("em-main", color_scheme.emphasis_main),
            get_color_css_variables("light", color_scheme.light),
            get_color_css_variables("dark", color_scheme.dark),
            get_color_css_variables("mode", color_scheme.mode),
            get_color_css_variables("warn", color_scheme.warn),
            get_color_css_variables("warn-main", color_scheme.warn_main),
            if theme_context_data.is_dark_mode { "dark" } else { "light" }
    };

    let document: Document = window().unwrap().document().unwrap().dyn_into().unwrap();

    let style_element = if let Some(existing_style) = document.query_selector("head style").unwrap()
    {
        existing_style.dyn_into::<web_sys::HtmlElement>().unwrap()
    } else {
        let new_style = document.create_element("style").unwrap();
        let new_style = new_style.dyn_into::<web_sys::HtmlElement>().unwrap();
        document
            .query_selector("head")
            .unwrap()
            .unwrap()
            .append_child(&new_style)
            .unwrap();
        new_style
    };

    style_element.set_inner_html(&style_content);

    rsx! {
        {&cx.children}
    }
}

fn get_color_css_variables(name: &str, hsl: Hsl) -> String {
    format!(
        "--{}-color: hsl({}, {}%, {}%);",
        name, hsl.hue, hsl.saturation, hsl.lightness
    )
}

fn generate_color_scheme(theme: Theme, is_dark_mode: bool) -> ColorScheme {
    let base = match theme {
        Theme::Default => Hsl::from_hsl(0.0, 0.0, 0.0),
        Theme::Readonly => Hsl::from_hsl(30.0, 100.0, 60.0),
        Theme::Dev => Hsl::from_hsl(200.0, 100.0, 30.0),
    };

    let background = if is_dark_mode {
        Hsl::from_hsl(base.hue, base.saturation, base.lightness * 0.8)
    } else {
        Hsl::from_hsl(base.hue, base.saturation, base.lightness * 1.2)
    };

    ColorScheme {
        main: base,
        background,
        emphasis: base,
        emphasis_main: base,
        warn: base,
        warn_main: base,
        light: base,
        dark: base,
        mode: base,
    }
}
