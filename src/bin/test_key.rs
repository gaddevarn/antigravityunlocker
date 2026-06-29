use sha2::{Sha256, Digest};

fn verify_key(key: &str) -> bool {
    let k: String = key.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    let k = k.to_uppercase();
    if k.len() != 24 { return false; }
    
    let mut hasher = Sha256::new();
    hasher.update(&k[..12]);
    let secret = "$7:y!]rf8lj+M?Cx<u)ypSbTk>I4nRc^";
    hasher.update(secret.as_bytes());
    let expected = hex::encode(hasher.finalize()).to_uppercase();
    let expected = &expected[..12];
    
    println!("Expected: {}", expected);
    println!("Actual  : {}", &k[12..]);
    
    let mut result = 0;
    for (x, y) in k[12..].chars().zip(expected.chars()) {
        result |= (x as u8) ^ (y as u8);
    }
    result == 0
}

fn main() {
    let res = verify_key("MY7SDFI09OXY8B56571B1ED8");
    println!("Result: {}", res);
}
