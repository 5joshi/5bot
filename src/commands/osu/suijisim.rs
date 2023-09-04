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
        ApplicationCommandExt, BatchGetResponse, EmbedBuilder, MessageBuilder, SPREADSHEET_BASE,
        SUIJI_SPREADSHEET_ID,
    },
};

#[command]
#[description = "Simulate a possible iteration of the suiji bracket"]
pub struct Suijisim;

async fn suijisim(ctx: Arc<Context>, command: ApplicationCommand) -> BotResult<()> {
    let req = format!(
        "{}{}/values:batchGet?ranges=Players!F5%3AF132&ranges=Players!M5%3AM132&ranges=Players!T5%3AT132&ranges=Players!AA5%3AAA132&key={}",
        SPREADSHEET_BASE,
        SUIJI_SPREADSHEET_ID,
        env::var("GOOGLE_API_KEY").expect("Missing environment variable (GOOGLE_API_KEY).")
    );
    let bytes = reqwest::get(req).await?.bytes().await?;
    info!("{}", String::from_utf8_lossy(&bytes));
    let mut response: BatchGetResponse = serde_json::from_slice(&bytes)?;
    let mut players = response
        .valueRanges
        .into_iter()
        .flat_map(|vr| vr.values.into_iter().flatten().collect::<Vec<_>>());

    let mut iter = players.step_by(2);
    let mut res = Vec::with_capacity(8);

    for _ in 0..8 {
        res.push(iter.by_ref().take(32).collect::<Vec<_>>());
    }

    let [a1, a2, b1, b2, c1, c2, d1, d2] = dbg!(res.as_mut_slice()) else {
        unreachable!()
    };
    {
        let mut rng = rand::thread_rng();

        a1.shuffle(&mut rng);
        a2.shuffle(&mut rng);
        b1.shuffle(&mut rng);
        b2.shuffle(&mut rng);
        c1.shuffle(&mut rng);
        c2.shuffle(&mut rng);
        d1.shuffle(&mut rng);
        d2.shuffle(&mut rng);
    }
    let mut fields1: Vec<EmbedField> = (0..16)
        .map(|idx| EmbedField {
            inline: true,
            name: format!("Team {}", idx + 1),
            value: format!(
                "```\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n```",
                a1[idx], a2[idx], b1[idx], b2[idx], c1[idx], c2[idx], d1[idx], d2[idx],
            ),
        })
        .collect();
    let mut fields2: Vec<EmbedField> = (16..32)
        .map(|idx| EmbedField {
            inline: true,
            name: format!("Team {}", idx + 1),
            value: format!(
                "```\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n```",
                a1[idx], a2[idx], b1[idx], b2[idx], c1[idx], c2[idx], d1[idx], d2[idx],
            ),
        })
        .collect();
    let embed1 = EmbedBuilder::new()
        .title("Random Suiji Simulation")
        .fields(fields1)
        .build();
    let embed2 = EmbedBuilder::new().fields(fields2).build();
    let builder = MessageBuilder::new().embed(embed1).embed(embed2);
    return command.create_message(&ctx, builder).await;
}
