// use std::{cell::RefCell, rc::Rc};

// use dioxus::prelude::*;

// #[derive(PartialEq, Clone, Debug)]
// struct AutocompleteProps<'a> {
//     options: Vec<&'a str>,
//     placeholder: &'a str,
//     on_select: EventHandler<String>,
// }

// // impl<'a> Properties for AutocompleteProps<'a> {
// //     // implement required methods
// // }

// #[component]
// // pub fn Autocomplete(props: AutocompleteProps) -> Element {
// //     // このstateはcontextにする
// //     let input_value = use_signal(|| String::new());
// //     let filtered_options = props
// //         .options
// //         .iter()
// //         .filter(|option| option.contains(input_value.read().as_str()))
// //         .cloned()
// //         .collect::<Vec<_>>();

// //     rsx! {
// //       div {
// //         class: "autocomplete-container",
// //         style: "position: relative; width: 100%;",
// //         // input {
// //         //     class: "autocomplete-input",
// //         //     style: "width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px;",
// //         //     placeholder: "{cx.props.placeholder}",
// //         //     value: "{input_value}",
// //         //     // oninput: move |evt| input_value.set(evt.value.clone()),
// //         // }
// //         // if !filtered_options.is_empty() {
// //         //     ul {
// //         //         class: "autocomplete-dropdown",
// //         //         style: "position: absolute; top: 100%; left: 0; right: 0; background: white; border: 1px solid #ccc; border-radius: 4px; max-height: 150px; overflow-y: auto; z-index: 1000;",
// //         //         filtered_options.iter().map(|option| rsx!(
// //         //             li {
// //         //                 class: "autocomplete-item",
// //         //                 style: "padding: 8px; cursor: pointer;",
// //         //                 onclick: move |_| {
// //         //                     props.on_select.call(option.to_string());
// //         //                     input_value.set(option.to_string());
// //         //                 },
// //         //                 "{option}"
// //         //             }
// //         //         ))
// //         //     }
// //         // }
// //         "あああああああああああ"
// //       }
// //     }
// // }
// #[component]
// pub fn Autocomplete() -> Element {
//     let mut input_value = use_signal(|| "".to_string());
//     let suggestions: Signal<Vec<&str>> = use_signal(|| vec!["Apple", "Banana", "Cherry"]);
//     // let mut filtered_suggestions: Signal<Vec<&str>> = use_signal(|| vec![]);
//     let filtered_suggestions = Rc::new(RefCell::new(vec![]));

//     // let on_input = move |event: FormEvent| {
//     //     let query = event.value().clone();
//     //     input_value.set(query.clone());
//     //     filtered_suggestions.set(
//     //         suggestions
//     //             .read()
//     //             .iter()
//     //             .filter(|&&item| item.to_lowercase().contains(&query.to_lowercase()))
//     //             .cloned() // 元の&strをクローン
//     //             .collect(),
//     //     );
//     // };
//     let on_input = {
//         // let filtered_suggestions = Rc::clone(&filtered_suggestions);
//         // let suggestions = Rc::clone(&suggestions);

//         move |event: FormEvent| {
//             let query = event.value().clone();
//             input_value.set(query.clone());

//             let filtered: Vec<&str> = suggestions
//                 .read()
//                 .iter()
//                 .filter(|&&item| item.to_lowercase().contains(&query.to_lowercase()))
//                 .cloned() // 元の&strをクローン
//                 .collect();
//             // .iter()
//             // .filter(|&&item| item.to_lowercase().contains(&query.to_lowercase()))
//             // .cloned()
//             // .collect();
//             // filtered_suggestions.set(
//             //     suggestions
//             //         .read()
//             //         .iter()
//             //         .filter(|&&item| item.to_lowercase().contains(&query.to_lowercase()))
//             //         .cloned() // 元の&strをクローン
//             //         .collect(),
//             // );

//             filtered_suggestions.borrow_mut().clear();
//             filtered_suggestions.borrow_mut().extend(filtered);
//         }
//     };

//     // let mut on_select = move |selection| {
//     //     input_value.set(selection);
//     //     filtered_suggestions.set(vec![]); // Clear suggestions
//     // };
//     let on_select = {
//         move |selection: &str| {
//             input_value.set(selection.to_string());
//             filtered_suggestions.borrow_mut().clear(); // Clear suggestions
//         }
//     };

//     rsx!(
//       div {
//         h1 { "Autocomplete Example" }
//         input {
//           value: "{input_value}",
//           oninput: on_input,
//           placeholder: "Type to search..."
//         }
//         ul {
//           {filtered_suggestions.borrow().iter().map(|&suggestion| rsx!{
//               li {
//                   key: "{suggestion}",
//                   onclick: move |_| on_select(suggestion),
//                   "{suggestion}"
//               }
//             }
//           )}
//         }
//       }
//     )
// }
