use chrono::{DateTime, Utc};
use twilight_model::channel::embed::{
    Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedThumbnail,
};

use crate::utils::{datetime::date_to_string, DARK_GREEN};

type EmbedFields = Vec<EmbedField>;

pub struct EmbedBuilder(Embed);

impl Default for EmbedBuilder {
    fn default() -> Self {
        Self(Embed {
            author: None,
            color: Some(DARK_GREEN),
            description: None,
            fields: Vec::new(),
            footer: None,
            image: None,
            kind: String::new(),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None,
        })
    }
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(mut self) -> Embed {
        self.0.kind.push_str("rich");

        self.0
    }

    pub fn author(mut self, author: impl Into<EmbedAuthor>) -> Self {
        self.0.author.replace(author.into());

        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.0.color.replace(color);

        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        let description = description.into();
        self.0.description.replace(description);

        self
    }

    pub fn fields(mut self, fields: EmbedFields) -> Self {
        self.0.fields = fields;

        self
    }

    pub fn footer(mut self, footer: impl Into<EmbedFooter>) -> Self {
        self.0.footer.replace(footer.into());

        self
    }

    pub fn image(mut self, image: impl Into<String>) -> Self {
        let url = image.into();

        if !url.is_empty() {
            let image = EmbedImage {
                height: None,
                width: None,
                proxy_url: None,
                url: Some(url),
            };

            self.0.image.replace(image);
        }

        self
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        let timestamp = date_to_string(&timestamp);
        self.0.timestamp.replace(timestamp);

        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.0.title.replace(title.into());

        self
    }

    pub fn thumbnail(mut self, thumbnail: impl Into<String>) -> Self {
        let url = thumbnail.into();

        if !url.is_empty() {
            let thumbnail = EmbedThumbnail {
                height: None,
                width: None,
                proxy_url: None,
                url: Some(url),
            };

            self.0.thumbnail.replace(thumbnail);
        }

        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url.replace(url.into());

        self
    }
}
