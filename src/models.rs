use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize, sqlx::FromRow)]
pub struct Visit
{
    pub id: i32,
    pub ip: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub score: i32,
    pub level: i32,
    pub name: Option<String>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct VisitRequest
{
    pub ip: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub score: i32,
    pub level: i32,
    pub name: Option<String>
}