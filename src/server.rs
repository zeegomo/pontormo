use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;

use crate::index::static_rocket_route_info_for_register;
use crate::index::static_rocket_route_info_for_welcome;
use crate::register::static_rocket_route_info_for_process;
use crate::register::static_rocket_route_info_for_process_google;
use rocket::http::Status;

use crate::config::Config;
use failure::Fallible;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::net::IpAddr;
use std::process;
use std::sync::{Arc, Mutex};

const ASSETS_PATH: &str = "/assets";
const ASSETS_DIR: &str = "assets";
const DEFAULT_OUTPUT: &str = "out.csv";

pub const ALREADY_REGISTERED: Status = Status {
    code: 491,
    reason: "Already registered",
};

pub struct Server {
    inner: Arc<ServerInner>,
}

pub struct ServerInner {
    renderer: ServerRenderer,
    ip: Mutex<HashSet<IpAddr>>,
    name: Mutex<HashSet<String>>,
    config: Config,
}

impl ServerInner {
    pub fn register_ip(&self, ip: IpAddr) -> Fallible<()> {
        let mut set = self.ip.lock().unwrap();
        if set.contains(&ip) {
            bail!("already registered ip");
        }
        set.insert(ip);
        Ok(())
    }

    pub fn register_name(&self, value: &str) -> Fallible<()> {
        let value = value.to_string();
        let mut set = self.name.lock().unwrap();
        if set.contains(&value) {
            bail!("already registered name");
        }
        set.insert(value);
        Ok(())
    }

    pub fn render(&self, path: &'static str, additional: Context) -> Template {
        self.renderer.render(path, additional)
    }

    pub fn check_domain(&self, mail: &str) -> Fallible<bool> {
        if let Some(google_provider) = &self.config.google_sign_in {
            Ok(google_provider.check_domain(mail))
        } else {
            bail!("google sign in unavailable")
        }
    }
}

#[derive(Clone)]
pub struct ServerRenderer {
    base: Context,
}

impl ServerRenderer {
    pub fn render(&self, path: &'static str, additional: Context) -> Template {
        let mut context = self.base.clone();
        context.extend(additional);
        Template::render(path, context)
    }

    pub fn new(config: &Config) -> Self {
        let mut context = Context::new();
        context.insert("color", &config.appearance.color);
        if let Some(google) = &config.google_sign_in {
            google.init_tera(&mut context);
            //println!("{:?}", google);
        }
        ServerRenderer { base: context }
    }
}

impl Server {
    pub fn new() -> Server {
        let config = Config::load().expect("could not load config file");
        Server {
            inner: Arc::new(ServerInner {
                renderer: ServerRenderer::new(&config),
                ip: Mutex::new(HashSet::new()),
                name: Mutex::new(HashSet::new()),
                config,
            }),
        }
    }

    pub fn launch(&self) {
        let inner = self.inner.clone();
        ctrlc::set_handler(move || Self::write_on_closing(&inner.name.lock().unwrap()))
            .expect("Error setting Ctrl-C handler");

        rocket::ignite()
            .mount("/", routes![register, welcome, process, process_google])
            .mount(ASSETS_PATH, StaticFiles::from(ASSETS_DIR))
            .attach(Template::fairing())
            .manage(self.inner.clone())
            .launch();
    }

    fn open_output_file() -> Fallible<File> {
        Ok(File::create(DEFAULT_OUTPUT)?)
    }

    fn inner_write_on_closing<T: std::io::Write>(
        mut wtr: csv::Writer<T>,
        data: &HashSet<String>,
    ) -> Fallible<()> {
        if data
            .iter()
            .map(|name| wtr.write_record(&[name]))
            .any(|res| res.is_err())
        {
            bail!("cannot save data")
        } else {
            Ok(())
        }
    }

    fn write_on_closing(data: &HashSet<String>) {
        let res = if let Ok(file) = Self::open_output_file() {
            Self::inner_write_on_closing(csv::Writer::from_writer(file), data)
        } else {
            Self::inner_write_on_closing(csv::Writer::from_writer(io::stdout()), data)
        };
        if res.is_err() {
            error!("Could not save data");
            process::exit(1);
        }
        process::exit(0);
    }
}
