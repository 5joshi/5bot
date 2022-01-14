use twilight_model::channel::embed::EmbedFooter;

#[derive(Clone)]
pub struct Footer(EmbedFooter);

impl Footer {
    pub fn new(text: impl Into<String>) -> Self {
        Self(EmbedFooter {
            text: text.into(),
            icon_url: None,
            proxy_icon_url: None,
        })
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        let icon_url = icon_url.into();
        self.0.icon_url.replace(icon_url);

        self
    }

    pub fn into_footer(self) -> EmbedFooter {
        self.0
    }

    pub fn as_footer(&self) -> &EmbedFooter {
        &self.0
    }
}

impl From<Footer> for EmbedFooter {
    fn from(footer: Footer) -> Self {
        footer.into_footer()
    }
}

impl From<&Footer> for EmbedFooter {
    fn from(footer: &Footer) -> Self {
        footer.as_footer().to_owned()
    }
}
