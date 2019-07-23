use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::prelude::*;
use diesel::prelude::*;
use std::error::Error;

use crate::schema::tokens;
use crate::schema::users;

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub hashed_password: String,
    pub rank: i32,
}

impl User {
    pub fn verify_password(
        conn: &PgConnection,
        username: &String,
        password: &String,
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::users::dsl;

        let user = dsl::users
            .filter(dsl::username.eq(username))
            .get_result::<Self>(conn)?;

        let hashed_password = hash(password, DEFAULT_COST).unwrap();

        Ok(verify(&user.hashed_password, &hashed_password).unwrap())
    }

    pub fn rank_for_username(
        conn: &PgConnection,
        username: &String,
    ) -> Result<i32, diesel::result::Error> {
        use crate::schema::users::dsl;

        let user = dsl::users
            .filter(dsl::username.eq(username))
            .get_result::<Self>(conn)?;

        Ok(user.rank)
    }

    pub fn create(
        conn: &PgConnection,
        username: &String,
        password: &String,
        rank: i32,
    ) -> Result<i64, diesel::result::Error> {
        use crate::schema::users::dsl;

        let user_id = diesel::insert_into(dsl::users)
            .values((
                dsl::username.eq(username),
                dsl::hashed_password.eq(&hash(password, DEFAULT_COST).unwrap()),
                dsl::rank.eq(rank),
            ))
            .returning(dsl::id)
            .get_result(conn)?;

        Ok(user_id)
    }
}

#[derive(Queryable)]
pub struct Token {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub created_when: chrono::DateTime<Utc>,
}

impl Token {
    pub fn generate(conn: &PgConnection, user_id: i64) -> Result<String, diesel::result::Error> {
        use crate::schema::tokens::dsl;
        let token = uuid::Uuid::new_v4().to_simple().to_string().to_lowercase();

        diesel::insert_into(dsl::tokens)
            .values((dsl::user_id.eq(user_id), dsl::token.eq(&token)))
            .execute(conn)?;

        Ok(token)
    }

    pub fn user_by_token(
        conn: &PgConnection,
        token: &String,
    ) -> Result<User, diesel::result::Error> {
        use crate::schema::tokens;
        use crate::schema::users;

        let token = tokens::dsl::tokens
            .filter(tokens::dsl::token.eq(token))
            .get_result::<Self>(conn)?;

        Ok(users::dsl::users
            .find(token.user_id)
            .get_result::<User>(conn)?)
    }
}
