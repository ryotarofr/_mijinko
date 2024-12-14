// use diamond_types::list::{ListBranch, ListOpLog};
// use dioxus::prelude::*;
// use dioxus_logger::tracing::{info, Level}; // Import the RleVec type
// use std::string::FromUtf8Error;

// /// TODO:
// /// diamond_typesのOpLogの構造体にが文字列部分(ins_content)をパースできる実装がない可能性が高い
// /// -> cloneしてimpl追加するしかないか？

// #[component]
// pub fn CrdtSample() -> Element {
//     let mut oplog = use_signal(ListOpLog::new); // OpLog のインスタンスを作成
//     let input_value = use_signal(String::new);

//     // ログインユーザのidをセット
//     let agent = oplog.write().get_or_create_agent_id("ryotarofr"); // AgentId を取得

//     // ボタンを押したときに処理を実行する
//     let on_add_text = move |_| {
//         // OpLog にデータを追加
//         oplog.write().add_insert(agent, 0, "Hello World!!!"); // "abc123" を挿入
//         oplog.write().add_delete_without_content(agent, 1..5); // 'b' を削除
//         oplog.write().add_insert(agent, 0, "ababab"); // "ababab" を挿入

//         // 結果を出力
//         info!("{:?}", oplog);
//     };

//     // match oplog.parse_ins_content_to_string() {
//     //     Ok(text) => println!("Parsed text: {}", text),
//     //     Err(e) => eprintln!("Error parsing ins_content: {:?}", e),
//     // }

//     rsx! {
//       div {
//         button { onclick: on_add_text, "Update CRDT" }
//         textarea { value: "{input_value}" }
//       }
//     }
// }
