mod models;
mod utils;

use dotenv::dotenv;
use models::{ArticleDB, ArticleRequest, Author, Endpoint};
use rusqlite::{Connection, Result};
use serde_json;
use std::fs;

use crate::utils::send_message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let conn = Connection::open("articles.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles (id INTEGER PRIMARY KEY, name TEXT, article_id TEXT, body TEXT, created_at TEXT, updated_at TEXT, edited_at TEXT)",
        (),
    )?;

    let authors_data =
        fs::read_to_string("./src/authors.json").expect("unable to read 'authors.json'");
    let authors: Vec<Author> =
        serde_json::from_str(&authors_data).expect("unable to parse 'authors.json'");

    let endpoints_data =
        fs::read_to_string("./src/endpoints.json").expect("unable to read 'endpoints.json'");
    let endpoints: Vec<Endpoint> =
        serde_json::from_str(&endpoints_data).expect("unable to parse 'endpoints.json'");

    for endpoint in endpoints.iter() {
        println!("[FETCHING] Fetching '{}'", &endpoint.name);
        let mut page = 1;

        loop {
            let url = format!("{}&page={}", &endpoint.url, page);

            let body = reqwest::get(&url)
                .await
                .unwrap()
                .json::<ArticleRequest>()
                .await
                .expect(format!("failed to request {}", &url).as_str());

            println!(
                "[+] Fetching {}/{} and got {} articles",
                page,
                body.page_count,
                body.articles.len()
            );

            for article in &body.articles {
                let sql_select =
                    format!("SELECT * FROM articles WHERE article_id = '{}'", article.id);
                let mut stmt = conn.prepare(&sql_select)?;
                let article_data = stmt.query_map([], |row| {
                    Ok(ArticleDB {
                        id: row.get(0)?,
                        article_id: row.get(1)?,
                        body: row.get(2)?,
                        name: row.get(3)?,
                        created_at: row.get(4)?,
                        updated_at: row.get(5)?,
                        edited_at: row.get(6)?,
                    })
                })?;

                if article_data.count() > 0 {
                    continue;
                }

                println!("[+] New article found {} -> {}", article.name, article.url);

                send_message(&article, &authors, &endpoint.name)
                    .await
                    .expect("error sending message");

                let article_db = ArticleDB {
                    id: 0,
                    article_id: article.id.to_string(),
                    body: article.body.clone(),
                    name: article.name.clone(),
                    created_at: article.created_at.clone(),
                    updated_at: article.updated_at.clone(),
                    edited_at: article.edited_at.clone(),
                };

                conn.execute(
                    "INSERT INTO articles (name, article_id, body, created_at, updated_at, edited_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    (&article_db.name, &article_db.article_id, &article_db.body, &article_db.created_at, &article_db.updated_at, &article_db.edited_at),
                )?;
            }

            if page >= body.page_count {
                break;
            } else {
                page += 1;
            }
        }
    }

    Ok(())
}
