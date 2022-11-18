mod models;
mod utils;

use dotenv::dotenv;
use models::{ArticleDB, ArticleRequest, Author, Endpoint};
use rusqlite::{params, Connection, Result};
use serde_json;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use tokio::main;

fn hasher<T>(obj: T) -> String
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish().to_string()
}

use crate::utils::{send_message_new, send_message_update};

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let conn = Connection::open("articles.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles (id INTEGER PRIMARY KEY, name TEXT, article_id TEXT, body TEXT, body_hash TEXT, created_at TEXT, updated_at TEXT, edited_at TEXT)",
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

            let body = match reqwest::get(&url)
                .await
                .unwrap()
                .json::<ArticleRequest>()
                .await
            {
                Ok(value) => value,
                Err(_) => {
                    eprintln!("failed to request {}", &url);
                    break;
                }
            };

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
                let article_data: Option<ArticleDB> = match stmt
                    .query_map([], |row| {
                        Ok(ArticleDB {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            article_id: row.get(2)?,
                            body: row.get(3)?,
                            body_hash: row.get(4)?,
                            created_at: row.get(5)?,
                            updated_at: row.get(6)?,
                            edited_at: row.get(7)?,
                        })
                    })?
                    .last()
                {
                    Some(art) => Some(art.unwrap()),
                    None => None,
                };

                if !article_data.is_none() {
                    let hash = hasher(article.body.clone());

                    if hash != article_data.as_ref().unwrap().body_hash {
                        println!("[*] Updated article {}", article.name);

                        send_message_update(
                            &article,
                            &article_data.as_ref().unwrap().body,
                            &authors,
                            &endpoint.name,
                        )
                        .await
                        .expect("error sending 'update article' message");

                        match conn.execute(
                            "UPDATE articles SET body = ?1, body_hash = ?2 WHERE article_id = ?3",
                            params![
                                &article.body,
                                &hasher(&article.body),
                                &article.id.to_string()
                            ],
                        ) {
                            Err(err) => panic!("error updating db: {}", err),
                            Ok(value) => value,
                        };
                    }

                    continue;
                }

                send_message_new(&article, &authors, &endpoint.name)
                    .await
                    .expect("error sending 'new article' message");

                println!("[+] New article found {} -> {}", article.name, article.url);

                let article_db = ArticleDB {
                    id: 0,
                    article_id: article.id.to_string(),
                    body: article.body.clone(),
                    body_hash: hasher(article.body.clone()),
                    name: article.name.clone(),
                    created_at: article.created_at.clone(),
                    updated_at: article.updated_at.clone(),
                    edited_at: article.edited_at.clone(),
                };

                conn.execute(
                    "INSERT INTO articles (name, article_id, body, body_hash, created_at, updated_at, edited_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    (&article_db.name, &article_db.article_id, &article_db.body,&article_db.body_hash, &article_db.created_at, &article_db.updated_at, &article_db.edited_at),
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
