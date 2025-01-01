use keyboard_types::Code;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// use once_cell::sync::Lazy;
///   - 1度だけ初期化し、その後は静的に利用
pub static KANA_MAP: Lazy<HashMap<Vec<Code>, &'static str>> = Lazy::new(|| {
    use Code::*;
    [
        // 母音
        (vec![KeyA], "あ"),
        (vec![KeyI], "い"),
        (vec![KeyU], "う"),
        (vec![KeyE], "え"),
        (vec![KeyO], "お"),
        // K行
        (vec![KeyK, KeyA], "か"),
        (vec![KeyK, KeyI], "き"),
        (vec![KeyK, KeyU], "く"),
        (vec![KeyK, KeyE], "け"),
        (vec![KeyK, KeyO], "こ"),
        // S行
        (vec![KeyS, KeyA], "さ"),
        (vec![KeyS, KeyI], "し"), // si
        (vec![KeyS, KeyU], "す"),
        (vec![KeyS, KeyE], "せ"),
        (vec![KeyS, KeyO], "そ"),
        // T行
        (vec![KeyT, KeyA], "た"),
        (vec![KeyT, KeyI], "ち"),       // ti
        (vec![KeyT, KeyU], "つ"),       // tu
        (vec![KeyT, KeyS, KeyU], "つ"), // tsu
        (vec![KeyT, KeyE], "て"),
        (vec![KeyT, KeyO], "と"),
        // N行
        (vec![KeyN, KeyA], "な"),
        (vec![KeyN, KeyI], "に"),
        (vec![KeyN, KeyU], "ぬ"),
        (vec![KeyN, KeyE], "ね"),
        (vec![KeyN, KeyO], "の"),
        // H行
        (vec![KeyH, KeyA], "は"),
        (vec![KeyH, KeyI], "ひ"),
        (vec![KeyH, KeyU], "ふ"), // hu
        (vec![KeyH, KeyE], "へ"),
        (vec![KeyH, KeyO], "ほ"),
        // M行
        (vec![KeyM, KeyA], "ま"),
        (vec![KeyM, KeyI], "み"),
        (vec![KeyM, KeyU], "む"),
        (vec![KeyM, KeyE], "め"),
        (vec![KeyM, KeyO], "も"),
        // Y行
        (vec![KeyY, KeyA], "や"),
        (vec![KeyY, KeyU], "ゆ"),
        (vec![KeyY, KeyO], "よ"),
        // R行
        (vec![KeyR, KeyA], "ら"),
        (vec![KeyR, KeyI], "り"),
        (vec![KeyR, KeyU], "る"),
        (vec![KeyR, KeyE], "れ"),
        (vec![KeyR, KeyO], "ろ"),
        // W行
        (vec![KeyW, KeyA], "わ"),
        (vec![KeyW, KeyO], "を"),
        // ン
        (vec![KeyN, KeyN], "ん"),
        // 濁音 G行
        (vec![KeyG, KeyA], "が"),
        (vec![KeyG, KeyI], "ぎ"),
        (vec![KeyG, KeyU], "ぐ"),
        (vec![KeyG, KeyE], "げ"),
        (vec![KeyG, KeyO], "ご"),
        // Z行(ざ行)
        (vec![KeyZ, KeyA], "ざ"),
        (vec![KeyZ, KeyI], "じ"), // zi
        (vec![KeyZ, KeyU], "ず"),
        (vec![KeyZ, KeyE], "ぜ"),
        (vec![KeyZ, KeyO], "ぞ"),
        // D行(だ行)
        (vec![KeyD, KeyA], "だ"),
        (vec![KeyD, KeyI], "ぢ"), // di
        (vec![KeyD, KeyU], "づ"), // du
        (vec![KeyD, KeyE], "で"),
        (vec![KeyD, KeyO], "ど"),
        // B行(ば行)
        (vec![KeyB, KeyA], "ば"),
        (vec![KeyB, KeyI], "び"),
        (vec![KeyB, KeyU], "ぶ"),
        (vec![KeyB, KeyE], "べ"),
        (vec![KeyB, KeyO], "ぼ"),
        // P行(ぱ行)
        (vec![KeyP, KeyA], "ぱ"),
        (vec![KeyP, KeyI], "ぴ"),
        (vec![KeyP, KeyU], "ぷ"),
        (vec![KeyP, KeyE], "ぺ"),
        (vec![KeyP, KeyO], "ぽ"),
        // 拗音(きゃ行など)
        // K行拗音
        (vec![KeyK, KeyY, KeyA], "きゃ"),
        (vec![KeyK, KeyY, KeyU], "きゅ"),
        (vec![KeyK, KeyY, KeyO], "きょ"),
        // G行拗音
        (vec![KeyG, KeyY, KeyA], "ぎゃ"),
        (vec![KeyG, KeyY, KeyU], "ぎゅ"),
        (vec![KeyG, KeyY, KeyO], "ぎょ"),
        // S行拗音(しゃ=shaとする)
        (vec![KeyS, KeyH, KeyA], "しゃ"),
        (vec![KeyS, KeyH, KeyU], "しゅ"),
        (vec![KeyS, KeyH, KeyO], "しょ"),
        // Z行拗音(じゃ=zya)
        (vec![KeyZ, KeyY, KeyA], "じゃ"),
        (vec![KeyZ, KeyY, KeyU], "じゅ"),
        (vec![KeyZ, KeyY, KeyO], "じょ"),
        // T行拗音(ちゃ=tya)
        (vec![KeyT, KeyY, KeyA], "ちゃ"),
        (vec![KeyT, KeyY, KeyU], "ちゅ"),
        (vec![KeyT, KeyY, KeyO], "ちょ"),
        // D行拗音(ぢゃ=dya)
        (vec![KeyD, KeyY, KeyA], "ぢゃ"),
        (vec![KeyD, KeyY, KeyU], "ぢゅ"),
        (vec![KeyD, KeyY, KeyO], "ぢょ"),
        // N行拗音(にゃ=nya)
        (vec![KeyN, KeyY, KeyA], "にゃ"),
        (vec![KeyN, KeyY, KeyU], "にゅ"),
        (vec![KeyN, KeyY, KeyO], "にょ"),
        // H行拗音(ひゃ=hya)
        (vec![KeyH, KeyY, KeyA], "ひゃ"),
        (vec![KeyH, KeyY, KeyU], "ひゅ"),
        (vec![KeyH, KeyY, KeyO], "ひょ"),
        // B行拗音(びゃ=bya)
        (vec![KeyB, KeyY, KeyA], "びゃ"),
        (vec![KeyB, KeyY, KeyU], "びゅ"),
        (vec![KeyB, KeyY, KeyO], "びょ"),
        // P行拗音(ぴゃ=pya)
        (vec![KeyP, KeyY, KeyA], "ぴゃ"),
        (vec![KeyP, KeyY, KeyU], "ぴゅ"),
        (vec![KeyP, KeyY, KeyO], "ぴょ"),
        // M行拗音(みゃ=mya)
        (vec![KeyM, KeyY, KeyA], "みゃ"),
        (vec![KeyM, KeyY, KeyU], "みゅ"),
        (vec![KeyM, KeyY, KeyO], "みょ"),
        // R行拗音(りゃ=rya)
        (vec![KeyR, KeyY, KeyA], "りゃ"),
        (vec![KeyR, KeyY, KeyU], "りゅ"),
        (vec![KeyR, KeyY, KeyO], "りょ"),
        // 他組み合わせ
        (vec![KeyT, KeyS, KeyA], "ツァ"),
        (vec![KeyT, KeyS, KeyI], "ツィ"),
        (vec![KeyT, KeyS, KeyA], "ツァ"),
        (vec![KeyT, KeyS, KeyE], "ツェ"),
        (vec![KeyT, KeyS, KeyO], "ツォ"),
    ]
    .into_iter()
    .collect()
});

// 特殊文字用のハッシュマップを静的に初期化
// TODO : IMEじゃない時に記号系は使えるからIMEで使うとしたら大文字記号にせなあかん
pub static SPECIAL_MAP_JIS: Lazy<HashMap<Vec<Code>, &'static str>> = Lazy::new(|| {
    use Code::*;
    [
        // Ctrl + Alt + KeyG で "γ" (ギリシャ文字ガンマ)を生成する場合
        (vec![ControlLeft, AltLeft, KeyG], "γ"),
        (vec![ControlLeft, AltRight, KeyG], "γ"),
        (vec![ControlRight, AltLeft, KeyG], "γ"),
        (vec![ControlRight, AltRight, KeyG], "γ"),
        // Shift + Key1 -> "!"
        (vec![ShiftLeft, Digit1], "!"),
        (vec![ShiftRight, Digit1], "!"),
        // Shift + Key2 -> '"'
        (vec![ShiftLeft, Digit2], "\""),
        (vec![ShiftRight, Digit2], "\""),
        // Shift + Key3 -> "#"
        (vec![ShiftLeft, Digit3], "#"),
        (vec![ShiftRight, Digit3], "#"),
        // Shift + Key4 -> "$"
        (vec![ShiftLeft, Digit4], "$"),
        (vec![ShiftRight, Digit4], "$"),
        // Shift + Key5 -> "%"
        (vec![ShiftLeft, Digit5], "%"),
        (vec![ShiftRight, Digit5], "%"),
        // Shift + Key6 -> "&"
        (vec![ShiftLeft, Digit6], "&"),
        (vec![ShiftRight, Digit6], "&"),
        // Shift + Key7 -> "'"
        (vec![ShiftLeft, Digit7], "\'"),
        (vec![ShiftRight, Digit7], "\'"),
        // Shift + Key8 -> "("
        (vec![ShiftLeft, Digit8], "("),
        (vec![ShiftRight, Digit8], "("),
        // Shift + Key9 -> ")"
        (vec![ShiftLeft, Digit9], ")"),
        (vec![ShiftRight, Digit9], ")"),
        // Shift + Equal -> "="
        (vec![ShiftLeft, Equal], "="),
        (vec![ShiftRight, Digit9], ")"),
        // Alt + KeyM で "µ" (マイクロ)
        // (vec![KeyAltLeft, KeyM], "µ"),
        // // 例: 3キーコンボで特定の記号を生成
        // (vec![KeyShiftLeft, KeyAltLeft, KeyL], "£"),
        // // 必要に応じてどんどん追加
        // (vec![KeyShiftLeft, KeyAltLeft, KeyE], "€"),
    ]
    .into_iter()
    .collect()
});
