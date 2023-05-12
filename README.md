# Safe Execution Protocol

## Overview

This codebase provides a protocol that serves as the foundation for a Proof-of-Capacity (PoC) concept. The primary objective is to ensure the secure and verifiable execution of a client-server interaction. This is achieved using digital signatures and hash functions which are part of modern cryptographic systems. 

The protocol is broken down into two main sections:

1. Client side operations
2. Server side operations

Let's look at both in detail.

## Client

The client performs the following operations:

1. **Reading and parsing the RSA private key**: The client begins by reading a private key from a PEM file. This key is used later to sign certain pieces of data.

```rust
let priv_pem = fs::read("client_private_key.pem").expect("Failed to read client private key");
let private_key = RsaPrivateKey::from_pkcs8_pem(&String::from_utf8(priv_pem).unwrap())
    .expect("Failed to parse private key");
```

2. Creating a SHA-256 hash of data: The client then reads certain data (in this case, from a file named "hdfs") and calculates its SHA-256 hash. This hash is later used to create a digital signature. The same process is repeated for the binary executable of the client itself.

```rust
let hdfs = fs::read("hdfs").expect("Failed to read hdfs");
let mut hasher = Hasher::new(MessageDigest::sha256()).unwrap();
hasher.update(&hdfs).unwrap();
let hdfs_sha256 = hasher.finish().unwrap();
```

3. Creating digital signatures: The client uses the private key to sign the hashes calculated in the previous step. These signatures are later sent to the server for verification.

```rust
let padding = Pkcs1v15Sign::new::<Sha256>();
let signature = private_key.sign(padding.clone(), &hdfs_sha256).unwrap();
let signature_client = private_key.sign(padding, &client_sha256).unwrap();
```

4. Sending data to the server: Finally, the client sends the signatures and the result of executing a command to the server. This is done using an HTTP POST request.

```rust
let client = Client::new();
let response = client.post("http://localhost:3030/verify")
    .json(&signed_data)
    .send()
    .await
    .expect("Failed to send data to server");
```

## Server

1. Reading and parsing the RSA public key: The server begins by reading a public key from a PEM file. This key is used to verify the signatures sent by the client.

```rust
let pem = fs::read("client_public_key.pem").expect("Failed to read client private key");
let public_key = RsaPublicKey::from_public_key_pem(&String::from_utf8(pem).unwrap())
    .expect("Failed to parse public key");
```

2. Verifying digital signatures: The server uses the public key to verify the signatures sent by the client. It compares these signatures against the known hashes of the data.


```rust
let padding = Pkcs1v15Sign::new::<Sha256>();
let result = public_key.verify(padding.clone(), &expected_hdfs_hash_bytes, &signed_data.signature);
```

3. Rewarding tokens: If the verification succeeds, the server rewards a certain number of tokens based on the result sent by the clien

```rust
let result_u32 = signed_data.result.trim().parse::<u32>().unwrap();
println!(
    "GB reported: {:?}, tokens rewarded: {}",
    result_u32,
    result
```