/// `cd` の引数（例: ".", "..", "dir1/dir2", "../..", etc.）をパースして
/// 現在のパス `current` を更新する関数の例。
pub fn apply_cd_command(current: &str, extracted: &str) -> String {
    // 現在のパスをスラッシュで分割し、空要素を排除してベクタにする
    // 例: "/home/user" => ["home", "user"]
    //     "home/user"  => ["home", "user"] (先頭スラッシュがない場合も考慮)
    let mut segments: Vec<String> = current
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    // "cd dir/subdir" のように `/` 区切りの複数ディレクトリが来る可能性もあるため、
    // extracted を `/` で分割してトークンごとに処理。
    for token in extracted.split('/') {
        match token {
            "" => {
                // 連続したスラッシュなどの場合: 例: "dir//subdir"
                // 特に何もせずスキップ
            }
            "." => {
                // "cd ." => カレントディレクトリを変更しない
            }
            ".." => {
                // "cd .." => 一つ上の階層へ
                segments.pop();
            }
            other => {
                // 通常のディレクトリ/ファイル名 => 追加
                segments.push(other.to_string());
            }
        }
    }

    // 結果として segments が空ならルート ("/") として扱うか、空文字にするかはお好みで
    if segments.is_empty() {
        // ここではルートディレクトリとして返す例
        "/".to_string()
    } else {
        // 先頭にスラッシュを付けたい場合
        format!("/{}", segments.join("/"))

        // 先頭にスラッシュを付けたくない場合 (相対パスとして扱うなら)
        // segments.join("/")
    }
}
