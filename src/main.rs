// Bulk form(To implement SHA256 hashing for bulk data using the ring crate)
use ring::digest;

fn sha256_digest(data: &[u8]) -> String {
    let digest = digest::digest(&digest::SHA256, data);
    let digest_hex = hex::encode(digest.as_ref());
    digest_hex
}

fn main() {
    let data = b"Your bulk data here"; // Replace with your bulk data
    let hashed_data = sha256_digest(data);

    println!("Original data: {:?}", data);
    println!("SHA256 Hash: {}", hashed_data);
}

// Chunk form(To implement SHA256 hashing for chunk data using the ring crate)
// use ring::digest;
// use hex;

// fn sha256_digest_chunked(data_chunks: Vec<&[u8]>) -> String {
//     let mut context = digest::Context::new(&digest::SHA256);

//     for chunk in data_chunks {
//         context.update(chunk);
//     }

//     let digest = context.finish();
//     hex::encode(digest.as_ref())
// }

// fn main() {
//     // Explicitly cast each string literal to a byte slice
//     let chunks = vec![
//         b"Chunk 1: The quick brown fox " as &[u8],
//         b"Chunk 2: jumps over the lazy dog" as &[u8],
//     ];

//     let hashed_data = sha256_digest_chunked(chunks);

//     println!("SHA256 Hash: {}", hashed_data);
// }

