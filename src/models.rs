#[derive(Queryable)]
pub struct Document {
    pub id: i32,
    pub hash: String,
    pub tei: String,
}
