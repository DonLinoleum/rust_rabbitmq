use axum::{Json, extract::State, http::StatusCode};

use crate::{AppState, models::{Visit, VisitRequest}};


pub async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<Visit>>, StatusCode>
{
    let result = sqlx::query_as::<_,Visit>("SELECT * FROM visits order by id DESC LIMIT 100")
        .fetch_all(&state.pool)
        .await;

        let visits = match result  {
            Ok(res ) => res,
            Err(e) => {eprintln!("{:?}",e); return Err( StatusCode::INTERNAL_SERVER_ERROR)}
        };
    Ok(Json(visits))
}

pub async fn add_visit(State(state): State<AppState>, Json(data): Json<VisitRequest>) -> Result<Json<Visit>, StatusCode>
{
    let visit = insert_visit(&state.pool, data).await;
        match visit {
            Ok(visit) => {
                println!("Visit: {:?}", visit);
                return Ok(Json(visit));
            },
            Err(e) => {
                eprintln!("{:?}",e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            } 
        }
}

pub async fn insert_visit(pool: &sqlx::PgPool, data: VisitRequest) -> Result<Visit, sqlx::Error>
{
        sqlx::query_as::<_,Visit>("insert into visits (ip, date, score, level, name) values ($1,COALESCE($2,NOW()),$3,$4,$5) returning *;")
        .bind(data.ip)
        .bind(data.date)
        .bind(data.score)
        .bind(data.level)
        .bind(data.name)
        .fetch_one(pool)
        .await
}