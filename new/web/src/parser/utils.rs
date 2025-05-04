/// \n で区切る関数
/// これは複数行のコピペや削除の時に使う(別ファイルに移動予定)
pub fn split_lines(input: &str) -> Vec<&str> {
    input.split('\n').collect()
}
