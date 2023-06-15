# Article Tracker

## A project by Datacord

> Fetches the support articles to see if there are new updates on the articles or if there are new articles

## Setup and run with Docker

```docker
docker build -t datacord-article-tracker:latest .
```

```docker
docker volume create datacord-article-tracker
```

```docker
docker run \
-e HC_ARTICLES_WEBHOOK_URL=REPLACE_ME \
-e DEV_HC_ARTICLES_WEBHOOK_URL=REPLACE_ME \
-e DMA_ARTICLES_WEBHOOK_URL=REPLACE_ME \
-e SAFETY_ARTICLES_WEBHOOK_URL=REPLACE_ME \
-e CREATOR_ARTICLES_WEBHOOK_URL=REPLACE_ME \
-e HC_ARTICLES_ROLE_ID=REPLACE_ME \
-e DEV_HC_ARTICLES_ROLE_ID=REPLACE_ME \
-e DMA_ARTICLES_ROLE_ID=REPLACE_ME \
-e SAFETY_ARTICLES_ROLE_ID=REPLACE_ME \
-e CREATOR_ARTICLES_ROLE_ID=REPLACE_ME \
-e DB_URL='./db/articles.db' \
--mount type=volume,src=datacord-article-tracker,target=/article-tracker/db \
--name article-tracker \
datacord-article-tracker:latest
```