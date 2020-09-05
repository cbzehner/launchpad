#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod db;
mod models;
mod routes;
mod server;

#[cfg(test)]
mod tests;

fn main() {
    server::rocket().launch();
}
