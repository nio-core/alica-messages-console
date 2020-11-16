use rand::Rng;
use sha2::Digest;

pub fn random_nonce() -> String {
    let mut nonce = [0u8; 16];
    rand::thread_rng().try_fill(&mut nonce).expect("Error filling nonce");
    data_encoding::HEXLOWER.encode(&nonce)
}

pub fn calculate_checksum<T>(data: &T) -> String
    where T: AsRef<[u8]> {
    let mut hasher = sha2::Sha512::new();
    hasher.update(data);
    data_encoding::HEXLOWER.encode(&hasher.finalize()[..])
}
