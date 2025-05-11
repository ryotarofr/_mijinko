use std::collections::HashMap;
use std::hash::Hash;

/// 任意のキー／値マップを受け取り、各エントリをインデックス付きで変換して
/// 同じキー付きの新しいマップを返す。
///
/// # 型パラメータ
/// - `K`: キーの型。ハッシュ可能かつ比較可能で、Clone できる必要があります。
/// - `V`: 元の値の型（参照で借りて読み取る想定）
/// - `R`: マッピング後の値の型
/// - `F`: マッパー関数の型。(`(&K, &V)`, `usize`) → `R`
///
/// # 引数
/// - `map`: 変換対象のマップへの参照
/// - `mut f`: 値を変換する関数。タプル `(&K, &V)` とエントリの順序 `index` を受け取ります。
///
/// # 戻り値
/// - 元のキーと、マッパー関数で生成された値からなる新しい `HashMap<K, R>`
pub fn get_mapped_object<K, V, R, F>(map: &HashMap<K, V>, mut f: F) -> HashMap<K, R>
where
    K: Eq + Hash + Clone,
    F: FnMut((&K, &V), usize) -> R,
{
    map.iter()
        .enumerate()
        .map(|(idx, (key, value))| {
            // key.clone(): 新しいマップに同じキーを所有権ごとコピー
            (key.clone(), f((key, value), idx))
        })
        .collect()
}
