use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub hashed_password: String,
    pub rank: i32,
}

impl User {
    pub fn verify_password(
        conn: &PgConnection,
        username: &str,
        password: &str,
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::users::dsl;

        let user = dsl::users
            .filter(dsl::username.eq(username))
            .get_result::<Self>(conn)?;

        Ok(verify(password, &user.hashed_password).unwrap())
    }

    pub fn by_username(conn: &PgConnection, username: &str) -> Result<Self, diesel::result::Error> {
        use crate::schema::users::dsl;

        dsl::users
            .filter(dsl::username.eq(username))
            .get_result::<Self>(conn)
    }

    pub fn create(
        conn: &PgConnection,
        username: &str,
        password: &str,
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
            .returning(dsl::token)
            .get_result(conn)
    }

    pub fn destroy(conn: &PgConnection, token: &str) -> Result<(), diesel::result::Error> {
        use crate::schema::tokens::dsl;

        diesel::delete(dsl::tokens.filter(dsl::token.eq(token)))
            .execute(conn)
            .map(|_| ())
    }

    pub fn user_by_token(conn: &PgConnection, token: &str) -> Result<User, diesel::result::Error> {
        use crate::schema::tokens::dsl;
        use crate::schema::users;

        let token = dsl::tokens
            .filter(dsl::token.eq(token))
            .get_result::<Self>(conn)?;

        Ok(users::dsl::users
            .find(token.user_id)
            .get_result::<User>(conn)?)
    }
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Profile {
    pub id: i64,
    pub machine_name: String,
    pub human_name: String,
    pub module: String,
    pub config: Option<serde_json::Value>,
}

impl Profile {
    pub fn list(conn: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::profiles::dsl;

        dsl::profiles.get_results::<Self>(conn)
    }

    pub fn id_for_machine_name(
        conn: &PgConnection,
        machine_name: &str,
    ) -> Result<i64, diesel::result::Error> {
        use crate::schema::profiles::dsl;

        let profile = dsl::profiles
            .filter(dsl::machine_name.eq(machine_name))
            .get_result::<Profile>(conn)?;

        Ok(profile.id)
    }
}

use crate::schema::reports;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Report {
    pub id: i64,
    pub user_id: i64,
    pub created_when: chrono::DateTime<Utc>,
    pub file_multihash: String,
    pub file: Option<Vec<u8>>,
}

impl Report {
    pub fn list_for_user(
        conn: &PgConnection,
        user_id: i64,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::reports::dsl;

        dsl::reports
            .filter(dsl::user_id.eq(user_id))
            .get_results::<Self>(conn)
    }

    pub fn create(
        conn: &PgConnection,
        user_id: i64,
        file: Vec<u8>,
    ) -> Result<i64, diesel::result::Error> {
        use crate::schema::reports::dsl;
        use multihash::{encode, Hash};

        diesel::insert_into(dsl::reports)
            .values((
                dsl::user_id.eq(user_id),
                dsl::file_multihash.eq(hex::encode(encode(Hash::SHA2256, &file).unwrap())),
                dsl::file.eq(Some(file)),
            ))
            .returning(dsl::id)
            .get_result(conn)
    }

    pub fn discard_file_check_user(
        conn: &PgConnection,
        user_id: i64,
        report_id: i64,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::reports::dsl;

        diesel::update(
            dsl::reports
                .find(report_id)
                .filter(dsl::user_id.eq(user_id)),
        )
        .set(dsl::file.eq::<Option<Vec<u8>>>(None))
        .execute(conn)
        .map(|_| ())
    }
}

use crate::schema::tasks;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
pub struct Task {
    id: i64,
    report_id: i64,
    profile_id: i64,
    created_when: chrono::DateTime<Utc>,
    completed_when: Option<chrono::DateTime<Utc>>,
    status: String,
}

impl Task {
    pub fn list_for_report(
        conn: &PgConnection,
        report_id: i64,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::tasks::dsl;

        dsl::tasks
            .filter(dsl::report_id.eq(report_id))
            .get_results::<Self>(conn)
    }

    pub fn create(
        conn: &PgConnection,
        report_id: i64,
        profile_id: i64,
    ) -> Result<i64, diesel::result::Error> {
        use crate::schema::tasks::dsl;
        diesel::insert_into(dsl::tasks)
            .values((
                dsl::report_id.eq(report_id),
                dsl::profile_id.eq(profile_id),
                dsl::status.eq("new"),
            ))
            .returning(dsl::id)
            .get_result(conn)
    }
}
