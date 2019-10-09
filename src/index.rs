use crate::server;
use rocket::State;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;
use server::ServerInner;
use std::sync::Arc;

#[get("/")]
pub fn welcome(state: State<Arc<ServerInner>>) -> Template {
    state.render("index", Context::new())
}

#[get("/register")]
pub fn register(state: State<Arc<ServerInner>>) -> Template {
    state.render("register", Context::new())
}
