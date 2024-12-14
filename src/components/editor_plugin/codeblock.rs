use dioxus::prelude::*;

pub fn CodeBlock(contents: String) -> Element {
    rsx! {
      div { style: "position: relative;",
        div { dangerous_inner_html: contents }
        button {
          style: "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
          "onclick": "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
          "Copy"
        }
      }
    }
}
