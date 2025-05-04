/// \n で区切る関数
/// これは複数行のコピペや削除の時に使う(別ファイルに移動予定)
pub fn split_lines(input: String) -> Vec<String> {
    input.split('\n').map(|s| s.to_string()).collect()
}
