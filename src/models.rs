use serde::{Deserialize, Serialize};

// src/authors.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub image: String,
}

// src/endpoints.json
#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub url: String,
    pub name: String,
}

// DB Article
#[derive(Debug)]
pub struct ArticleDB {
    pub id: usize,
    pub name: String,
    pub article_id: String,
    pub body: String,
    pub body_hash: String,
    pub created_at: String,
    pub updated_at: String,
    pub edited_at: String,
}

// Zendesk Article API

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleRequest {
    pub count: i64,
    pub next_page: Option<String>,
    pub page: i64,
    pub page_count: i64,
    pub per_page: i64,
    pub previous_page: Option<String>,
    pub articles: Vec<Article>,
    pub sort_by: String,
    pub sort_order: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: i64,
    pub url: String,
    pub html_url: String,
    pub author_id: i64,
    pub comments_disabled: bool,
    pub draft: bool,
    pub promoted: bool,
    pub position: i64,
    pub vote_sum: i64,
    pub vote_count: i64,
    pub section_id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub title: String,
    pub source_locale: String,
    pub locale: String,
    pub outdated: bool,
    pub outdated_locales: Vec<String>, //? could be wrong
    pub edited_at: String,
    pub user_segment_id: Option<String>, //? could be wrong
    pub permission_group_id: i64,
    pub label_names: Vec<String>,
    pub body: String,
}
