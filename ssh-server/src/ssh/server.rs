use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use russh::{
    Channel, ChannelId, CryptoVec, Pty,
    keys::PublicKey,
    server::{self, Auth, Config, Handle, Msg, Session, Server as _},
    Error as SshError,
};

use crate::ssh::{auth::AuthLog, terminal::TerminalHandle};
use crate::ssh::app::App;

#[derive(Clone, Debug)]
pub struct SshServer {
    pub clients: Arc<Mutex<HashMap<usize, (ChannelId, Handle, App)>>>,
    pub id: usize,
    pub auth_log: Arc<AuthLog>,
    pub protocol: Option<String>
}

impl SshServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: 0,
            auth_log: Arc::new(AuthLog::new()),
            protocol: None
        }
    }

    pub async fn run(
        mut self,
        config: Arc<Config>,
        addr: (&str, u16)
    ) -> Result<(), SshError> {
        self.run_on_address(config, addr).await?;
        Ok(())
    }
}

impl server::Server for SshServer {
    type Handler = Self;
    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        s
    }
    fn handle_session_error(&mut self, _error: <Self::Handler as server::Handler>::Error) {
        eprintln!("Session error: {:#?}", _error);
    }
}

impl server::Handler for SshServer {
    type Error = russh::Error;

    async fn auth_publickey(
        &mut self,
        username: &str,
        key: &PublicKey,
    ) -> Result<Auth, Self::Error> {
        let allowed_key_type = self.auth_log.eval_key(key).await;
        let comment = key.comment();

        log::info!(
        "Authentication attempt | user: {username}, key_type: {allowed_key_type}, comment: \"{comment}\""
    );

        self.protocol = Some(username.to_string());
        self.auth_log.record_key(username, key).await;

        Ok(Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let channel_id = channel.id();
        let handle = session.handle();
        let prot = self.protocol.as_deref();

        log::info!("Channel open session: {:?}", prot);
        let app = if prot == Some("tui") {
            let terminal_handle = TerminalHandle::start(handle.clone(), channel_id).await;
            let mut app = App::start_tui(terminal_handle);
            app.serve(None);
            app
        } else {
            App::start()
        };

        self.clients.lock().await.insert(self.id, (channel_id, handle, app));
        Ok(true)
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _term: &str,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        _modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        log::info!(
            "PTY request: {}x{} ({}x{} pixels)", col_width, row_height, pix_width, pix_height
        );
        session.channel_success(channel)?;
        let mut clients = self.clients.lock().await;

        if let Some((_chan_id, _handle, app)) = clients.get_mut(&self.id) {
            app.resize(col_width as u16, row_height as u16);
            app.serve(None); // render pty size
        }

        session.channel_success(channel)?;
        Ok(())
    }

    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut clients = self.clients.lock().await;
        if let Some((_chan_id, _handle, app)) = clients.get_mut(&self.id) {
            // Route based on protocol
            let route = self.protocol.as_deref();
            app.serve(route);

            session.data(channel, CryptoVec::from(app.content.clone()))?;
            session.channel_success(channel)?;
        } else {
            session.data(channel, CryptoVec::from("Session not found.\n"))?;
            session.close(channel)?;
        }

        Ok(())
    }

    async fn window_change_request(
        &mut self,
        _: ChannelId,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &mut Session,
    ) -> Result<(), Self::Error> {
        log::info!("Window resized: {}x{}", col_width, row_height);

        let mut clients = self.clients.lock().await;

        if let Some((_chan_id, _handle, app)) = clients.get_mut(&self.id) {
            app.resize(col_width as u16, row_height as u16);
            app.serve(None); // trigger re-render after resize
        }

        Ok(())
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut clients = self.clients.lock().await;

        if let Some((_chan_id, _handle, app)) = clients.get_mut(&self.id) {
            let should_exit = app.handle_input(data);

            if should_exit {
                // Send clear screen escape sequence
                let clear: &[u8] = b"\x1b[2J\x1b[H\r\n";
                session.data(channel, CryptoVec::from(clear))?;

                clients.remove(&self.id);
                session.close(channel)?;
                log::info!("Client close connection: {}", self.id);
                log::info!("{:?}", self);
            }
        }
        Ok(())
    }
}

impl Drop for SshServer {
    fn drop(&mut self) {
        let id = self.id;
        let clients = self.clients.clone();
        tokio::spawn(async move {
            let mut clients = clients.lock().await;
            clients.remove(&id);
        });
    }
}
