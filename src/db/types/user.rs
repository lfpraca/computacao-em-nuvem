use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user)]
pub struct UserInsert<'a> {
    pub phone_number: &'a str,
    pub name: &'a str,
    pub pass_hash: &'a str,
    pub address: &'a str,
    pub role: i16,
}
