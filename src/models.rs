use schema::documents;

#[derive(Queryable)]
pub struct Document {
    pub id: i32,
    pub hash: String,
    pub tei: String,
}

#[insertable_into(documents)]
pub struct NewDocument<'a> {
    pub hash: &'a str,
    pub tei: &'a str,
}
