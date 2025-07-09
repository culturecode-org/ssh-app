#[derive(Debug)]
pub struct App {
    pub content: String,
}

impl App {
    pub fn start() -> Self {
        Self { content: String::new() }
    }

    pub fn default(&mut self) {
        let msg = "Here is the discord link: discord.gg/12345\r\n";
        self.content = msg.to_string();
    }

    pub fn hello_world(&mut self) {
        let msg = "Shell started! Hello World!\r\n";
        self.content = msg.to_string();
    }

    pub fn discord_login(&mut self) {
        let msg = "Here is the discord link: discord.gg/12345\r\n";
        self.content = msg.to_string();
    }

    pub fn serve(&mut self, route: Option<&str>) {
        match route {
            Some("hello") => self.hello_world(),
            Some("discord") => self.discord_login(),
            _ => self.default()
        }
    }
}
