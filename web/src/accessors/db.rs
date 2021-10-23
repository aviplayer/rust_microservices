extern crate common;

use deadpool_postgres::{Client, Config, Pool, PoolError};
use deadpool_postgres::tokio_postgres::{Error, Row};
use log::{error, info};
use tokio_postgres::tls::NoTls;

const INSERT_USER: &str = "insert into users
(id, login, node_id, avatar_url, site_admin)
values($1, $2, $3, $4, $5)
returning id";

const SELECT_USERS: &str = "select id, login, node_id, avatar_url, site_admin
from users";
const SELECT_USER: &str = "
where id=$1";

pub trait DbErr {}

#[derive(Debug)]
pub enum DbError {
    DatabaseError(String),
}

#[derive(Clone)]
pub struct Db {
    pub connection: Option<Pool>,
}

impl DbErr for DbError {}

impl Db {
    pub fn new(pool: Pool) -> Self { Db { connection: Some(pool) } }

    async fn get_client_immutable(&self) -> Result<Client, DbError>
    {
        let conn = match &self.connection {
            Some(pool) => {
                Ok(pool)
            }
            None => {
                error!("Connection doesn't exist!");
                Err(DbError::DatabaseError(String::from("Connection doesn't exist!")))
            }
        };
        match conn {
            Ok(pool) => {
                match pool.get().await {
                    Ok(client) => Ok(client),
                    Err(error) => {
                        error!("For some reasons can't get connection: {}", error);
                        Err(DbError::DatabaseError(String::from("Can't get connection!")))
                    }
                }
            }
            Err(err) =>
                Err(err)
        }
    }

    fn record_from_future(row: Row) -> common::User {
        common::User {
            id: row.get(0),
            login: row.get(1),
            node_id: row.get(2),
            avatar_url: row.get(3),
            site_admin: row.get(4),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<common::User>, Error> {
        let client = match self.get_client_immutable().await {
            Ok(client) => client,
            Err(err) => panic!(err)
        };
        let stmt = client.prepare(SELECT_USERS).await?;
        let rows = client.query(&stmt, &[]).await?;
        let mut result: Vec<common::User> = Vec::new();
        {
            for row in rows {
                let record = Db::record_from_future(row);
                result.push(record);
            }
        }
        Ok(result)
    }

    pub async fn get_user(&self, id: i32) -> Result<common::User, Error> {
        let client = match self.get_client_immutable().await {
            Ok(client) => client,
            Err(err) => panic!(err)
        };
        let select_one_query = format!("{}{}", SELECT_USERS, SELECT_USER);
        let stmt = client.prepare(&select_one_query).await?;
        let db_record = client.query_one(&stmt, &[&id]).await?;
        let row = Db::record_from_future(db_record);
        Ok(row)
    }

    pub async fn insert_user(&self, user: common::User) -> Result<i32, DbError> {
        let client = match self.get_client_immutable().await {
            Ok(client) => client,
            Err(err) => {
                error!("Can't get client {:?}", err);
                panic!(err);
            }
        };
        let stmt = client.prepare(INSERT_USER).await.unwrap();
        let rows = match client.query(&stmt,
                                      &[&user.id, &user.login, &user.node_id, &user.avatar_url, &user.site_admin])
            .await {
            Ok(rows) => rows,
            Err(err) => {
                error!("Can't insert result {}", err);
                error!("Face problems with inserting user {:?}", user);
                vec![]
            }
        };
        if rows.len() == 0 {
            return Err(DbError::DatabaseError(String::from("Can't insert user!")));
        }

        let id: i32 = rows[0].get(0);
        info!("Successfully created user: \n{:?} with id: {}", user, id);
        Ok(id)
    }
}
