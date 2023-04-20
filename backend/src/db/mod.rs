pub mod schema;
pub mod sequences{
    use diesel::sql_function;
    use diesel::sql_types::{ Text};
    sql_function! { fn nextval(x: Text) -> BigInt; }
    pub mod user_id{
        use diesel::{PgConnection, RunQueryDsl, select};
        pub(crate) fn next_val(conn: &mut PgConnection) -> i64{
            select(super::nextval("user_id")).get_result(conn).unwrap()
        }
    }


}
pub mod models;


