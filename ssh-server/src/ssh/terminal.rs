use std::io::{self, Write};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use russh::{ChannelId, server::Handle};

#[derive(Debug)]
pub struct TerminalHandle {
    sender: UnboundedSender<Vec<u8>>,
    sink: Vec<u8>,
}

impl TerminalHandle {
    pub async fn start(handle: Handle, channel: ChannelId) -> Self {
        let (sender, mut receiver) = unbounded_channel::<Vec<u8>>();
        tokio::spawn(async move {
            while let Some(data) = receiver.recv().await {
                let _ = handle.data(channel, data.into()).await;
            }
        });
        Self { sender, sink: Vec::new() }
    }
}

impl Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.sender.send(self.sink.clone()).map_err(|e| {
            io::Error::new(io::ErrorKind::BrokenPipe, e.to_string())
        })?;
        self.sink.clear();
        Ok(())
    }
}
