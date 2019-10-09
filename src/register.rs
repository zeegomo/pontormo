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

#[derive(FromForm, Debug)]
pub struct GoogleFormData {
    name: String,
    mail: String,
}

#[post("/process_google", data = "<form>")]
pub fn process_google(
    sock: SocketAddr,
    form: Result<Form<GoogleFormData>, FormError>,
    state: State<Arc<ServerInner>>,
) -> HttpResult<Template> {
    let (name, mail) = parse_google_form(form)?;
    let res = register_inner(sock, &name, Some(&mail), &state);

    match res {
        Err(server::ALREADY_REGISTERED) => Ok(already_registered(&state)),
        _ => res,
    }
}

#[post("/process", data = "<form>")]
pub fn process(
    sock: SocketAddr,
    form: Result<Form<FormData>, FormError>,
    state: State<Arc<ServerInner>>,
) -> HttpResult<Template> {
    let name = parse_form(form)?;
    let res = register_inner(sock, &name, None, &state);

    match res {
        Err(server::ALREADY_REGISTERED) => Ok(already_registered(&state)),
        _ => res,
    }
}

fn register_inner(
    sock: SocketAddr,
    name: &str,
    domain: Option<&str>,
    state: &State<Arc<ServerInner>>,
) -> HttpResult<Template> {
    if let Some(domain) = domain {
        if !state.check_domain(domain).map_err(to_ie_status)? {
            return Err(to_wrong_domain_status(()));
        }
    }

    state.register_ip(sock.ip()).map_err(to_ar_status)?;
    state.register_name(&name).map_err(to_ar_status)?;

    let mut context = Context::new();
    context.insert("name", &name);
    Ok(state.render("registered", context))
}

fn already_registered(state: &State<Arc<ServerInner>>) -> Template {
    state.render("error", Context::new())
}

fn parse_form(form: Result<Form<FormData>, FormError>) -> HttpResult<String> {
    match form {
        Ok(form) => Ok(format!("{} {}", form.name.clone(), form.surname.clone())),
        _ => Err(Status::UnprocessableEntity),
    }
}

fn parse_google_form(
    form: Result<Form<GoogleFormData>, FormError>,
) -> HttpResult<(String, String)> {
    println!("{:?}", form);
    match form {
        Ok(form) => Ok((form.name.clone(), form.mail.clone())),
        _ => Err(Status::UnprocessableEntity),
    }
}

fn to_ar_status<U>(_err: U) -> Status {
    server::ALREADY_REGISTERED
}

fn to_wrong_domain_status<U>(_err: U) -> Status {
    server::ALREADY_REGISTERED
}

fn to_ie_status<U>(_err: U) -> Status {
    Status::InternalServerError
}
