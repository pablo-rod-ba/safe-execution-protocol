
use serde::{Serialize, Deserialize};
use std::{env,fs,process::Command};
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

#[derive(Serialize, Deserialize, Debug)]
struct SignedData {
    //session_public_key: Vec<u8>,
    result: String,
    signature: Vec<u8>,
    client_bin_signature: Vec<u8>,
}
use reqwest::Client;

#[tokio::main]
async fn main() {
       let priv_pem = fs::read("client_private_key.pem").expect("Failed to read client private key");
       let private_key = RsaPrivateKey::from_pkcs8_pem(&String::from_utf8(priv_pem).unwrap())
           .expect("Failed to parse private key");

         // Guarda la clave privada de sesión en un archivo
       let hdfs = fs::read("hdfs").expect("Failed to read hdfs");
       let mut hasher = Hasher::new(MessageDigest::sha256()).unwrap();
       hasher.update(&hdfs).unwrap();
       let hdfs_sha256 = hasher.finish().unwrap();


       let client_path = env::current_exe().expect("Failed to get binary path");
       let client = fs::read(&client_path).expect("Failed to read binary");
        hasher = Hasher::new(MessageDigest::sha256()).unwrap();
       hasher.update(&client).unwrap();
       let client_sha256 = hasher.finish().unwrap();

       let padding = Pkcs1v15Sign::new::<Sha256>();
       println!("hdfs hash: {:?} / vec: {:?} ", hex::encode(hdfs_sha256.to_vec()),hdfs_sha256.to_vec());
       println!("client hash: {:?} / vec: {:?} ", hex::encode(client_sha256.to_vec()),client_sha256.to_vec());


       let signature = private_key.sign(padding.clone(), &hdfs_sha256).unwrap();
       let signature_client = private_key.sign(padding, &client_sha256).unwrap();

       println!("signature to vec: {:?}", signature.to_vec());

       println!("signature_client to vec: {:?}", signature_client.to_vec());



       let output = Command::new("./hdfs")
        .arg("")
        .output()
        .expect("failed to execute process");
       
       // Empaquetar los datos firmados
       let signed_data = SignedData {
       //result: hex::encode(binary_sha256.to_vec()).into(),
        result: String::from_utf8_lossy(&output.stdout).to_string(),
        signature,
        client_bin_signature: signature_client,
    };

        // Envía los datos firmados al servidor utilizando HTTP
         let client = Client::new();
        let response = client.post("http://localhost:3030/verify")
            .json(&signed_data)
            .send()
            .await
            .expect("Failed to send data to server");

        if response.status().is_success() {
            println!("Verification successful!");
        } else {
            eprintln!("Verification failed.");
        } 
}
