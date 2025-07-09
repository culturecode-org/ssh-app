use std::{fs, path::Path};
use rand_core::OsRng;
use russh::keys::{self, PrivateKey, PublicKey, Algorithm, PublicKeyBase64};

pub fn load_keypair(dir: &Path) -> Result<PrivateKey, String> {
    let priv_path = dir.join("private_key.pem");

    if priv_path.exists() {
        match keys::load_secret_key(&priv_path, None) {
            Ok(private_key) => {
                log_fingerprint(&private_key, "Loaded existing");
                return Ok(private_key);
            }
            Err(e) => log::warn!("Failed to load existing private key: {:?}", e),
        }
    }

    generate_and_store_keypair(dir)
}

fn generate_and_store_keypair(dir: &Path) -> Result<PrivateKey, String> {
    let private_key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)
        .map_err(|e| format!("Key generation failed: {:?}", e))?;
    let public_key = PublicKey::from(&private_key);
    log_fingerprint(&private_key, "Generated new");

    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let priv_path = dir.join("private_key.pem");
    let mut priv_file = fs::File::create(&priv_path).map_err(|e| e.to_string())?;
    keys::encode_pkcs8_pem(&private_key, &mut priv_file).map_err(|e| e.to_string())?;

    let pub_path = dir.join("public_key.pub");
    let pub_key_str = format!("{}", public_key.public_key_base64());
    fs::write(pub_path, pub_key_str).map_err(|e| e.to_string())?;

    Ok(private_key)
}

pub fn log_fingerprint(private_key: &PrivateKey, msg: &str) {
    let fingerprint = PublicKey::from(private_key).fingerprint(Default::default());
    log::info!("{} keypair with fingerprint: {}", msg, fingerprint);
}
