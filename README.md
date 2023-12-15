# Setup instructions
1. Download the rust programming language
    [`rust-lang.org/tools/install`](https://www.rust-lang.org/tools/install)
2. Install cargo-leptos
    `cargo install --locked cargo-leptos`
3. Run the program
    `cargo leptos watch`
4. Go to `localhost:3000` and et voila you have a magnificent web page

# Dependencies
`leptos` is used the front end and the back end integration
`actix` is used as the http server
`cargo-leptos` is used for building and running the project for both client and server at once
`mongodb` is used for managing the mongodb database

# Project structure
All the source files are in `src/`
`src/main.rs` setup the server http server
`src/lib.rs` has the function `hydrate` wich is runned on the client
`src/app/` contains the front-end as well as the joins between the server and the client
`src/database/` contains a class wich manages all interactions with the database
`src/model.rs` contains the schemas of the database
Others files in `src/` are just utils