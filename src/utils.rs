use rocket::http::Status;
use rocket::response::content::Content;
use rocket::http::ContentType;

use failure::Fallible;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

type HttpResult<T> = Result<T, Status>;

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    color: String,
    text_color: String,
}

fn load_as_string(filename: PathBuf) -> Fallible<String> {
    let mut buffer = String::new();
    File::open(filename)?.read_to_string(&mut buffer)?;

    Ok(buffer)
}

#[get("/colours.css")]
pub fn colours_css(
) -> HttpResult<Content<String>> {
    let buffer: String = load_as_string(PathBuf::from(crate::server::DEFAULT_CONFIG)).unwrap();
    let config: Config = toml::from_str(&buffer).unwrap_or(Config{
        color: String::from("#fff"),
        text_color: String::from("#000"),
    });

    Ok(Content(ContentType::CSS, format!("body{{\n\tbackground-color: {};\n\tcolor: {}!important\n}}", &config.color, &config.text_color)))
}
