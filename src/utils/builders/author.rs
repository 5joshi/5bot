use twilight_model::channel::embed::EmbedAuthor;

#[derive(Clone)]
pub struct Author(EmbedAuthor);

impl Author {
    pub fn new(name: impl Into<String>) -> Self {
        Self(EmbedAuthor {
            name: Some(name.into()),
            url: None,
            icon_url: None,
            proxy_icon_url: None,
        })
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url.replace(url.into());

        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        let icon_url = icon_url.into();
        self.0.icon_url.replace(icon_url);

        self
    }

    pub fn into_author(self) -> EmbedAuthor {
        self.0
    }

    pub fn as_author(&self) -> &EmbedAuthor {
        &self.0
    }
}

impl From<Author> for EmbedAuthor {
    fn from(author: Author) -> Self {
        author.into_author()
    }
}

impl From<&Author> for EmbedAuthor {
    fn from(author: &Author) -> Self {
        author.as_author().to_owned()
    }
}
