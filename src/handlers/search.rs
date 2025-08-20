use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[get("/search")]
pub async fn search(query: web::Query<SearchQuery>) -> impl Responder {
    let q = query.q.to_lowercase();
    // TODO: Query ke DB (user, transaksi, dsb) pakai LIKE/ILIKE
    // Dummy response
    let results = vec![
        format!("Hasil pencarian untuk: {}", q)
    ];
    HttpResponse::Ok().json(results)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(search);
}
