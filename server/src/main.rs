use hex;
use openssl::hash::{Hasher, MessageDigest};
use rsa::pkcs1v15::SigningKey;
use rsa::sha2::Sha256;
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::{
    pkcs1v15::Pkcs1v15Sign,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::{env, fs};
use warp::Filter;

#[derive(Serialize, Deserialize, Debug)]
struct SignedData {
    result: String,
    signature: Vec<u8>,
    client_bin_signature: Vec<u8>,
}

async fn verify_data(signed_data: SignedData) -> Result<impl warp::Reply, warp::Rejection> {
    // Hash registrado del resultado del comando
    let expected_hdfs_hash = "29d06356f87e3975266d091cbe37466ea9c97624c25f5fec1e14f50c22e89a6e";

    // Hash registrado del binario que llama al comando
    let expected_client_hash = "8821ae02315d4039ff1ba3dbf9208bbb229211d066af972df8ecf5e3cf9e74fb";

    let pem = fs::read("client_public_key.pem").expect("Failed to read client private key");
    let public_key = RsaPublicKey::from_public_key_pem(&String::from_utf8(pem).unwrap())
        .expect("Failed to parse public key");

    let expected_hdfs_hash_bytes = hex::decode(expected_hdfs_hash).unwrap();
    let expected_client_hash_bytes = hex::decode(expected_client_hash).unwrap();

    let padding = Pkcs1v15Sign::new::<Sha256>();
    let result = public_key.verify(padding.clone(), &expected_hdfs_hash_bytes, &signed_data.signature);

    match result {
        Ok(()) => {
            let result_client = public_key.verify(
                padding,
                &expected_client_hash_bytes,
                &signed_data.client_bin_signature,
            );

            match result_client {
                Ok(()) => {
                    let result_u32 = signed_data.result.trim().parse::<u32>().unwrap();
                    println!(
                        "GB reported: {:?}, tokens rewarded: {}",
                        result_u32,
                        result_u32 * 20000
                    );
                    return Ok(warp::http::StatusCode::OK);
                }
                Err(e) => {
                    eprintln!("Client Verification failed: {:?}", e);
                    return Ok(warp::http::StatusCode::BAD_REQUEST);
                }
            }
        }
        Err(e) => {
            eprintln!("Command Verification failed: {:?}", e);
            return Ok(warp::http::StatusCode::BAD_REQUEST);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting server...");
    let verify_route = warp::post()
        .and(warp::path("verify"))
        .and(warp::body::json())
        .and_then(verify_data);

    warp::serve(verify_route).run(([127, 0, 0, 1], 3030)).await;
}
