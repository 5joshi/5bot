use crate::BotResult;
use dashmap::DashSet;
use futures::stream::StreamExt;
use irc::client::prelude::*;

pub struct IrcClient {
    // Tracked users
    pub targets: DashSet<String>,

    // Online users
    pub online: DashSet<String>,
}

impl IrcClient {
    #[inline]
    pub fn new(targets: DashSet<String>) -> Self {
        IrcClient {
            targets,
            online: DashSet::new(),
        }
    }

    pub async fn run(
        &self,
        server: &str,
        port: u16,
        nickname: &str,
        password: &str,
    ) -> BotResult<()> {
        let config = Config {
            server: Some(server.to_owned()),
            port: Some(port),
            password: Some(password.to_owned()),
            nickname: Some(nickname.to_owned()),
            channels: vec!["#osu".to_owned()],
            use_tls: Some(false),
            ..Default::default()
        };

        let mut client = Client::from_config(config).await?;
        client.identify()?;

        let mut stream = client.stream()?;

        info!("[IRC] Connected to Bancho");

        while let Some(msg) = stream.next().await.transpose()? {
            match msg.command {
                Command::JOIN(..) => {
                    if let Some(Prefix::Nickname(mut name, ..)) = msg.prefix {
                        name.make_ascii_lowercase();

                        if self.targets.contains(&name) {
                            info!("[IRC] {} now online", name);

                            self.online.insert(name);
                        }
                    }
                }
                Command::QUIT(..) => {
                    if let Some(Prefix::Nickname(mut name, ..)) = msg.prefix {
                        name.make_ascii_lowercase();

                        if self.online.remove(&name).is_some() {
                            info!("[IRC] {} now offline", name,);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
