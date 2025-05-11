// export type RequiredFields<T, K extends keyof T> = T & Required<Pick<T, K>>;
// T の全フィールドを持つが、K で指定したフィールドだけは optional でなく必須にする
pub struct RequiredFields<T, K: std::hash::Hash + Eq> {
    pub data: T,
    pub required: std::collections::HashSet<K>,
}
impl<T, K: std::hash::Hash + Eq> RequiredFields<T, K> {
    pub fn new(data: T, required: std::collections::HashSet<K>) -> Self {
        Self { data, required }
    }
}
