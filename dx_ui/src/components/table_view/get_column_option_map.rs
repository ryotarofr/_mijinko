use component::table_view::use_sort::SortOrder;
use dioxus::prelude::*;

/**
 * 描画オプション指定用データの構造定義。
 *
 * いくつかの指定方法を持つ。
 *
 * - 列名のみを指定
 * ```
 * { key: "列1" }
 * ```
 *
 * - 列について詳細設定を指定
 * ```
 * {
 *   key: {
 *     label: "列1",
 *     valueMapper: (raw) => `値: ${raw}`,
 *     // ...
 *   },
 * }
 * ```
 */
// export type RenderMap<T> = (
//   [T] extends [never]
//     ? unknown
//     : { [Key in keyof T]?: ReactNode | ColumnOptionArgs<T, T[Key]> }
// ) & Record<`_${string}`, RequiredFields<ColumnOptionArgs<T, unknown>, "valueMapper">>;
/// マップ中の “普通の” 値を表す enum
#[derive(Clone)]
pub enum RenderEntry<T> {
    /// 直接ノードを返す
    Node(ReactNode),
    /// ColumnOptionArgs<T, 実際のフィールド値>
    Args(Box<ColumnOptionArgs<T, Box<dyn Any>>>),
}

/// アンダースコア付きキー専用に value_mapper を必須にしたエイリアス
pub type PrefixedArgs<T> = ColumnOptionArgs<T, Box<dyn Any>>;

pub struct RenderMap<T> {
    /// T のフィールド名に対応させる通常マップ
    pub entries: HashMap<String, RenderEntry<T>>,
    /// `_foo` のようなキーに対応させるマップ（value_mapper 必須）
    pub prefixed: HashMap<String, PrefixedArgs<T>>,
}

impl<T> RenderMap<T> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            prefixed: HashMap::new(),
        }
    }
}

pub struct ColumnOptionArgs<T, V> {
    /// ヘッダーへの描画内容
    /// pub label: VNode<'a>, Element のライフタイムが内部的にどうなっているかわかんね。
    pub label: Element,
    /// 各セルへの描画内容
    /// memo: dyn は デフォルトで `static がつく
    pub value_mapper: Option<Box<dyn Fn(&V, &RenderOptions<T>) -> Element>>,
    // pub valueMapper?: (
    //   rawValue: Value,
    //   options: RenderOptions<T>
    // ) -> Element,
    /**
     * true ならセルをヘッダーとして描画する。
     * デフォルトは`false`。
     *
     * 連続して true に設定された列はグループ化された行ヘッダーになる。
     */
    pub isRowHeader: Option<ReadOnlySignal<bool>>,
    /** ソート時に用いる比較関数。昇順。戻り値がマイナスならソートされず、プラスならソートされる。 */
    pub asc_sorter: Option<Box<dyn Fn(V, V) -> usize>>,
    /**
     * 初期描画時のソート順序。
     *
     * デフォルトは`"none"`(ソートなし)。
     */
    pub initSortOrder: Option<SortOrder>,
    /**
     * trueならソート順序を切り替え可能。
     *
     * デフォルトは`true`。
     */
    pub sortOrderIsChangeable: Option<ReadOnlySignal<bool>>,
    /**
     * trueなら列を描画しない。
     *
     * デフォルトは`false`。
     */
    pub isHidden: Option<ReadOnlySignal<bool>>,
    /**
     * 行の幅の初期値。親の css[grid-template-columns] に初期設定される。
     * 例えば、`"max-content"`を設定すれば最大コンテンツ幅に縮小される。
     *
     * デフォルトは`"minmax(max-content, 1fr)"`。
     */
    pub initColumnWidth: Option<ReadOnlySignal<String>>,
    /**
     * 左右中央寄せ設定。
     *
     * デフォルトは`"left"`だが、特定の型[数値, 日付] については`"right"`になる。
     */
    pub align: Option<Align>,
    /**
     * trueなら合計値を描画。
     *
     * デフォルトは`false`。
     */
    pub total: Option<ReadOnlySignal<bool>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Center,
    Left,
    Right,
}

pub struct RenderOptions<T> {
    pub data: T,
    pub id: String,
    pub isFocused: bool,
    pub isSelected: bool,
    pub dataIndex: usize,
    pub renderIndex: usize,
    pub localIndex: usize,
}
