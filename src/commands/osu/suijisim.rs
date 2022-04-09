use std::{env, sync::Arc, time::Instant};

use rand::{prelude::SliceRandom, thread_rng};
use twilight_model::{
    application::{
        callback::{CallbackData, InteractionResponse},
        interaction::ApplicationCommand,
    },
    channel::embed::EmbedField,
};

use crate::{
    context::Context,
    error::BotResult,
    utils::{
        ApplicationCommandExt, BatchGetResponse, EmbedBuilder, MessageBuilder, BST_SPREADSHEET_ID,
        SPREADSHEET_BASE,
    },
};

#[command]
#[description = "Simulate a possible iteration of the suiji bracket"]
pub struct Suijisim;

async fn suijisim(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
    let req = format!(
        "{}{}/values:batchGet?ranges=Players!H9%3AH72&key={}",
        SPREADSHEET_BASE,
        BST_SPREADSHEET_ID,
        env::var("GOOGLE_API_KEY").expect("Missing environment variable (GOOGLE_API_KEY).")
    );
    let bytes = reqwest::get(req).await?.bytes().await?;
    // info!("{}", String::from_utf8_lossy(&bytes));
    let mut response: BatchGetResponse = serde_json::from_slice(&bytes)?;
    let mut req_players = response
        .valueRanges
        .pop()
        .map(|vr| vr.values.into_iter().flatten().collect::<Vec<_>>());

    if let Some(mut players) = req_players {
        let mut iter = players.chunks_exact_mut(16);

        let a = iter.next();
        let b = iter.next();
        let c = iter.next();
        let d = iter.next();
        match (a, b, c, d) {
            (Some(a), Some(b), Some(c), Some(d)) => {
                {
                    let mut rng = rand::thread_rng();

                    a.shuffle(&mut rng);
                    b.shuffle(&mut rng);
                    c.shuffle(&mut rng);
                    d.shuffle(&mut rng);
                }
                let mut fields: Vec<EmbedField> = (0..8)
                    .map(|idx| EmbedField {
                        inline: true,
                        name: format!("Team {}", idx + 1),
                        value: format!(
                            "```\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n```",
                            a[idx * 2],
                            a[idx * 2 + 1],
                            b[idx * 2],
                            b[idx * 2 + 1],
                            c[idx * 2],
                            c[idx * 2 + 1],
                            d[idx * 2],
                            d[idx * 2 + 1],
                        ),
                    })
                    .collect();
                let builder = EmbedBuilder::new()
                    .title("Random Suiji Simulation")
                    .fields(fields);
                return command.create_message(&ctx, builder).await;
            }
            _ => {
                let builder =
                    MessageBuilder::new().error("Some issue occurred when retrieving playernames!");
                return command.create_message(&ctx, builder).await;
            }
        }
    } else {
        let builder =
            MessageBuilder::new().error("Some issue occurred when retrieving playernames!");
        return command.create_message(&ctx, builder).await;
    }
}
