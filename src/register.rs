use crate::server;
use crate::server::ServerInner;
use rocket::http::Status;
use rocket::request::{Form, FormError};
use rocket::State;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;
use std::net::SocketAddr;
use std::sync::Arc;

type HttpResult<T> = Result<T, Status>;

#[derive(FromForm)]
pub struct FormData {
    name: String,
    surname: String,
}

#[post("/register", data = "<form>")]
pub fn register(
    sock: SocketAddr,
    form: Result<Form<FormData>, FormError>,
    state: State<Arc<ServerInner>>,
) -> HttpResult<Template> {
    let res = register_inner(sock, form, &state);

    match res {
        Err(server::ALREADY_REGISTERED) => Ok(already_registered(&state)),
        _ => res,
    }
}

fn register_inner(
    sock: SocketAddr,
    form: Result<Form<FormData>, FormError>,
    state: &State<Arc<ServerInner>>,
) -> HttpResult<Template> {
    let (name, surname) = parse_form(form)?;

    state.register_ip(sock.ip()).map_err(to_status)?;
    state.register_name(&name, &surname).map_err(to_status)?;

    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("surname", &surname);
    Ok(state.render("registered", context))
}

fn already_registered(state: &State<Arc<ServerInner>>) -> Template {
    state.render("error", Context::new())
}

fn parse_form(form: Result<Form<FormData>, FormError>) -> HttpResult<(String, String)> {
    match form {
        Ok(form) => Ok((form.name.clone(), form.surname.clone())),
        _ => Err(Status::UnprocessableEntity),
    }
}

fn to_status<U>(_err: U) -> Status {
    server::ALREADY_REGISTERED
}
