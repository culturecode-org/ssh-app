mod ssh;

use std::{sync::Arc, path::Path};
use russh::{Preferred,server::Config};
use ssh::{server::SshServer, keypair};

#[tokio::main]
async fn main() {
    let host: &str = "0.0.0.0";
    let port: u16 = 2222;

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let private_key = match keypair::load_keypair(Path::new("./keypair")) {
        Ok(k) => k,
        Err(e) => {
            log::error!("Keypair error: {}", e);
            std::process::exit(1);
        }
    };

    let config = Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(3),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![private_key],
        preferred: Preferred {
            ..Preferred::default()
        },
        ..Default::default()
    };

    let config = Arc::new(config);
    let server = SshServer::new();

    log::info!("SSH server running at {}:{}", host, port);
    server.run(config, (host, port)).await.unwrap();
}
