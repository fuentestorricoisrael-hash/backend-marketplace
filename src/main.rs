use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;

#[derive(Serialize, sqlx::FromRow)]
struct Tienda {
    id: i32,
    nombre_comercial: String,
    numero_local: String,
    telefono_whatsapp: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL no encontrada en .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Conectado a PostgreSQL local con éxito.");

    let app = Router::new()
        .route("/api/tiendas", get(obtener_tiendas))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn obtener_tiendas(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Tienda>>, axum::http::StatusCode> {
    let tiendas = sqlx::query_as::<_, Tienda>(
        "SELECT id, nombre_comercial, numero_local, telefono_whatsapp FROM tiendas",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tiendas))
}
