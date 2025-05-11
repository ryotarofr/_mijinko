// const orders = ["none", "asc", "desc"] as const;
// type Order = typeof orders[number];
// export type { Order as SortOrder };
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    None,
    Asc,
    Desc,
}

pub struct SortArgs<V> {
    pub prev: V,
    pub next: V,
}

// 配列として使いたい場合
// pub const ORDERS: [SortOrder; 3] = [SortOrder::None, SortOrder::Asc, SortOrder::Desc];
