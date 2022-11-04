use crate::DBPool;

use {
    crate::util::*,
    crate::wallet::*,
    actix_web::{
        get, post,
        web::{Data, Json, Path},
        HttpResponse,
    },
    uuid::Uuid,
};

#[get("/wallets")]
pub async fn list_wallets(pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    let wallets: Vec<Wallet> = fetch_all_wallets(&conn);
    ResponseType::Ok(wallets).get_response()
}

#[get("/wallets/{id}")]
pub async fn get_wallet(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    let wallet: Option<Wallet> =
        fetch_wallet_by_id(Uuid::parse_str(path.0 .0.as_str()).unwrap(), &conn);
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Wallet/Club not found".to_string()))
            .get_response(),
    }
}

#[post("/wallets")]
pub async fn create_wallet(
    wallet_request: Json<NewWalletRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    let conn = crate::get_connection_to_pool(pool);
    match create_new_wallet(wallet_request.0, &conn) {
        Ok(created_wallet) => ResponseType::Created(created_wallet).get_response(),
        Err(_) => {
            ResponseType::NotFound(NotFoundMessage::new("Error creating wallet.".to_string()))
                .get_response()
        }
    }
}
