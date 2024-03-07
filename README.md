# Rusty shortener

A simple url shortener api, based on [pocket link](https://github.com/velascoandres/pocket-link) shortener platform.

## Stack
* sqlx
* actix-web
* actix-web-validator
* dotenv


## Setting env
Just paste your database url (postgres)

```env
DATABASE_URL=""
```


## Run in watch mode

```bash
 cargo watch -x run
```