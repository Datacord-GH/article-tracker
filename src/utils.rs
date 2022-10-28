use chrono::{DateTime, Utc};
use regex::Regex;
use serenity::model::prelude::Embed;
use serenity::prelude::SerenityError;
use serenity::utils::Colour;
use serenity::{http::Http, model::webhook::Webhook};
use std::env;

use crate::models::{Article, Author};

fn time_formatting(time: &String) -> String {
    let time = time
        .parse::<DateTime<Utc>>()
        .expect("error formating in 'fn time_formatting'");

    format!("<t:{}:F> (<t:{}:R>)", time.timestamp(), time.timestamp())
}

fn emoji_format(val: &bool) -> &str {
    match val {
        true => "<:greenTick:851441548922847262>",
        false => "<:redTick:851441548994412614>",
    }
}

fn format_ping(id: String) -> String {
    format!("<@&{}>", id)
}

fn send_info(article_type: &str) -> (String, String, String, Colour) {
    match article_type {
        "support" => (
            env::var("HC_ARTICLES_WEBHOOK_URL").expect("missing 'HC_ARTICLES_WEBHOOK_URL' in .env"),
            format_ping(
                env::var("HC_ARTICLES_ROLE_ID").expect("missing 'HC_ARTICLES_ROLE_ID' in .env"),
            ),
            "Support Article".to_string(),
            Colour::from_rgb(7, 180, 255),
        ),
        "dev" => (
            env::var("DEV_HC_ARTICLES_WEBHOOK_URL")
                .expect("missing 'DEV_HC_ARTICLES_WEBHOOK_URL' in .env"),
            format_ping(
                env::var("DEV_HC_ARTICLES_ROLE_ID")
                    .expect("missing 'DEV_HC_ARTICLES_ROLE_ID' in .env"),
            ),
            "Developer Support Article".to_string(),
            Colour::from_rgb(255, 255, 0),
        ),
        "discordmoderatoracademy" => (
            env::var("DMA_ARTICLES_WEBHOOK_URL")
                .expect("missing 'DMA_ARTICLES_WEBHOOK_URL' in .env"),
            format_ping(
                env::var("DMA_ARTICLES_ROLE_ID").expect("missing 'DMA_ARTICLES_ROLE_ID' in .env"),
            ),
            "Moderator Academy Article".to_string(),
            Colour::from_rgb(31, 139, 76),
        ),
        "safety" => (
            env::var("SAFETY_ARTICLES_WEBHOOK_URL")
                .expect("missing 'SAFETY_ARTICLES_WEBHOOK_URL' in .env"),
            format_ping(
                env::var("SAFETY_ARTICLES_ROLE_ID")
                    .expect("missing 'SAFETY_ARTICLES_ROLE_ID' in .env"),
            ),
            "Safety Article".to_string(),
            Colour::from_rgb(189, 133, 133),
        ),
        _ => panic!("Invalid article type"),
    }
}

pub async fn send_message(
    article: &Article,
    authors: &Vec<Author>,
    article_type: &String,
) -> Result<(), SerenityError> {
    let (token, ping, hook_name, color) = send_info(article_type);

    let http = Http::new("token");

    let webhook = Webhook::from_url(&http, &token).await?;

    let md_regex = Regex::new(r"(<([^>]+)>)").unwrap();
    let author_pos = authors
        .iter()
        .position(|r| r.id == article.author_id.to_string());

    let mut author = &Author {
        id: article.author_id.to_string(),
        image: "https://cdn.discordapp.com/embed/avatars/0.png".to_string(),
        name: "Unknown".to_string(),
    };

    if author_pos.is_some() {
        author = authors.get(author_pos.unwrap()).unwrap();
    }

    let embed = Embed::fake(|e| {
        e.title(&article.name)
            .url(&article.html_url)
            .field("Draft", emoji_format(&article.draft), true)
            .field("Promoted", emoji_format(&article.promoted), true)
            .field(
                "Comments Disabled",
                emoji_format(&article.comments_disabled),
                true,
            )
            .field("Created At", time_formatting(&article.created_at), true)
            .field("Updated At", time_formatting(&article.updated_at), true)
            .field("Edited At", time_formatting(&article.edited_at), true)
            .description(
                md_regex
                    .replace_all(&article.body, "")
                    .chars()
                    .take(250)
                    .collect::<String>(),
            )
            .footer(|f| f.text(&article.label_names.join(", ")))
            .author(|a| {
                a.name(format!("{} ({})", &author.name, &author.id))
                    .icon_url(&author.image)
            })
            .color(color)
    });

    webhook
        .execute(&http, true, |w| {
            w.content(ping).username(hook_name).embeds(vec![embed])
        })
        .await?;

    Ok(())
}
