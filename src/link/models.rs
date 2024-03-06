use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;



#[derive(Deserialize, Debug, Clone, sqlx::FromRow, Serialize)]
#[sqlx(rename_all = "camelCase")]
pub struct Link {
   pub id: i32,
   pub name: String,

   pub original_link: String,
   pub path: String,

   pub created_at: NaiveDateTime,
   pub updated_at: NaiveDateTime
}


#[derive(Deserialize, Debug, Clone, Validate)]
pub struct CreateLink {
  #[validate(length(min = 2, message = "Name must be greater than 2 chars"))]
   pub name: String,

   #[serde(default)]
   #[validate(length(min = 2))]
   pub description: Option<String>,
   #[validate(url)]
   pub original_link: String,

   #[serde(default)]
   #[validate(length(min = 3, message = "Path must be greater than 3 chars"))]
   pub path: Option<String> 
}


#[derive(Deserialize, Debug, Clone, Validate)]

pub struct UpdateLink {
    #[validate(length(min = 2, message = "Name must be greater than 2 chars"))]
    pub name: Option<String>,

    #[serde(default)]
    #[validate(length(min = 2, message = "Description must be greater than 2 chars"))]
    pub description: Option<String>,
    
    #[serde(default)]
    #[validate(url)]
    pub original_link: Option<String>,
    
    #[serde(default)]
    #[validate(length(min = 3, message = "Path must be greater than 3 chars"))]
    pub path: Option<String> 
 }


 #[derive(Deserialize, Debug, Clone, Validate)]
 pub struct SearchLink {
    #[serde(default)]
    #[validate(length(min = 1))]
    pub search: Option<String>,

    #[serde(default)]
    #[validate(range(min=1))]
    pub page: Option<i64>
 }



 impl Link {
   pub fn generate_unique_path () -> String {
      let binding = uuid::Uuid::new_v4().to_string();
      let uuid_str = binding.as_str().get(0..8);
  
      String::from(uuid_str.unwrap())
  }
 }