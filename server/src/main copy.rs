use rsa::{pkcs8::DecodePublicKey, PaddingScheme, RsaPublicKey};
use serde::{Serialize, Deserialize};
use std::fs;
use warp::Filter;


#[derive(Serialize, Deserialize, Debug)]
struct SignedData {
    session_public_key: Vec<u8>,
    result: String,
    client_signature: Vec<u8>,
    session_signature: Vec<u8>,
}

async fn verify_data(signed_data: SignedData) -> Result<impl warp::Reply, warp::Rejection> {
    // Verificar la firma del cliente utilizando la clave pública del cliente conocida
    let client_public_key_pem = fs::read("client_public_key.pem").expect("Failed to read client public key");
    let client_public_key = RsaPublicKey::from_pkcs1_pem(&String::from_utf8(client_public_key_pem).unwrap()).unwrap();
    let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256));
    
    

    if client_public_key.verify(padding, signed_data.result.as_bytes(), &signed_data.client_signature).is_ok() {
        // Verificar la firma de sesión utilizando la clave pública de sesión
        let session_public_key = RsaPublicKey::from_pkcs1_pem(&String::from_utf8(signed_data.session_public_key).unwrap()).unwrap();
        let padding_session = PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256));
        if session_public_key.verify(padding_session, signed_data.result.as_bytes(), &signed_data.session_signature).is_ok() {
            println!("Verification successful!");
            return Ok(warp::http::StatusCode::OK);
        }
    }

    eprintln!("Verification failed.");
    Ok(warp::http::StatusCode::BAD_REQUEST)
}

#[tokio::main]
async fn main() {
    println!("Starting server...");
    let verify_route = warp::post()
        .and(warp::path("verify"))
        .and(warp::body::json())
        .and_then(verify_data);

    warp::serve(verify_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
