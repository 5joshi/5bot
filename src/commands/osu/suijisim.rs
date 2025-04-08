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
        "{}{}/values:batchGet?ranges=Registrations!N10:N137&ranges=Registrations!N138:N265&ranges=Registrations!N266:N393&ranges=Registrations!N394:N521&key={}",
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

    let mut iter = players.step_by(1);
    let mut res = Vec::with_capacity(4);

    for _ in 0..4 {
        res.push(iter.by_ref().take(64).collect::<Vec<_>>());
    }

    let [a, b, c, d] = dbg!(res.as_mut_slice()) else {
        unreachable!()
    };
    {
        let mut rng = rand::thread_rng();

        a.shuffle(&mut rng);
        b.shuffle(&mut rng);
        c.shuffle(&mut rng);
        d.shuffle(&mut rng);
    }
    let mut fields1: Vec<EmbedField> = (0..16)
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
    let mut fields2: Vec<EmbedField> = (16..32)
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
    let embed1 = EmbedBuilder::new()
        .title("Random Suiji Simulation")
        .fields(fields1)
        .build();
    let embed2 = EmbedBuilder::new().fields(fields2).build();
    let builder = MessageBuilder::new().embed(embed1).embed(embed2);
    return command.create_message(&ctx, builder).await;
}
