use crate::schema;
use crate::models::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub fn get_user_pass(pg: &mut PgConnection, user_name: String) -> String {
    use schema::users::dsl::*;

    // let mut pg = pool.get().unwrap();

    let user = users
        .filter(name.eq(user_name))
        .limit(1)
        .select(User::as_select())
        .load(pg)
        .expect("No user found");

    return user[0].name.clone() + &user[0].pass.clone()
}