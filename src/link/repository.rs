use std::sync::Arc;

use serde::Serialize;
use sqlx::Error;

use crate::{db::ConnectionPool, errors::CustomError};

use super::models::{CreateLink, Link, SearchLink};


#[derive(Debug, Serialize)]
pub struct FetchLinkResponse {
    pub data: Vec<Link>,
    pub total: i64,
}

#[derive(Clone)]
pub struct LinkRepository {
    conn: Arc<ConnectionPool>,
}

impl LinkRepository {
    pub fn new(conn: ConnectionPool) -> Self {
        Self {
            conn: Arc::new(conn),
        }
    }

    fn get_conn(&self) -> &ConnectionPool{
        &self.conn
    }


    pub async fn find_by_path(&self, path: &String) -> Result<Option<Link>, CustomError>{
        let fetch_result = sqlx::query_as::<_, Link>("Select * FROM \"public\".\"Link\" WHERE path = $1 ")
        .bind(path)
        .fetch_optional(self.get_conn()).await;
    
        if let Ok(link) = fetch_result {
            return Ok(link)
        }
    
        match fetch_result {
            Ok(link) => Ok(link),
            Err(Error::RowNotFound) => Err(CustomError::NotFound(format!("Link not found by path: {path}"))),
            Err(_) => Err(CustomError::InternalError(format!("Error on fetching by id: {path}")))
        } 
    }


    pub async fn create(&self, create_link: &CreateLink) -> Result<Link, CustomError> {
        let path: String;
        let conn = &*self.conn;


        if let Some(link_path) = create_link.path.clone() {
            let fech_result = self.find_by_path(&link_path).await.unwrap();

            if let Some(link) = fech_result {
                return Err(CustomError::BadRequest(format!(
                    "Link already exists for path {0}",
                    link.path
                )));
            }

            path = link_path;
        } else {
            path = Link::generate_unique_path();
        }


        let result =
            sqlx::query("INSERT INTO \"Link\" (name, \"originalLink\", path) values ($1, $2, $3)")
                .bind(create_link.name.clone())
                .bind(create_link.original_link.clone())
                .bind(&path)
                .execute(conn)
                .await;

        if result.is_err() {
            println!("Error on create link {:?}", result.err());

            return Err(CustomError::InternalError(format!(
                "Error on create link: {0}",
                create_link.original_link
            )));
        }

        let new_link_result = self.find_by_path(&path).await;

        if let Ok(Some(new_link)) = new_link_result {
            return Ok(new_link);
        }

        Err(CustomError::InternalError(String::from(
            "Error on fecth link",
        )))
    }

    pub async fn find_many(
        &self,
        query: SearchLink,
    ) -> Result<FetchLinkResponse, CustomError> {
        let condition = "WHERE (name LIKE '%'|| $1 ||'%' OR $1 IS NULL)";
        let count_query = format!("SELECT COUNT(id) FROM \"public\".\"Link\" {condition}");
        let search_query = format!("SELECT * FROM \"public\".\"Link\" {condition} Limit 10 OFFSET (10 * (COALESCE($2, 1) - 1))");
    
        let fecth_result = sqlx::query_as::<_, Link>(search_query.as_str())
            .bind(&query.search)
            .bind(query.page.unwrap_or(1))
            .fetch_all(self.get_conn()).await;
    
        let count_result = sqlx::query_scalar::<_, i64>(count_query.as_str())
            .bind(&query.search)
            .fetch_one(self.get_conn())
            .await;
    
        if count_result.is_err() {
            println!("Db error: {:?}", count_result.err().unwrap());
    
            return Err(CustomError::InternalError(String::from(
                "Error on fetching",
            )));
        }
    
        if fecth_result.is_err() {
            println!("Db error: {:?}", fecth_result.err().unwrap());
    
            return Err(CustomError::InternalError(String::from(
                "Error on fetching",
            )));
        }
    
        let links = fecth_result.unwrap();
        let count = count_result.unwrap();
    
        Ok(FetchLinkResponse {
            data: links,
            total: count
        })
    }
    

}
