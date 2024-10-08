mod basic;
pub mod consts;
mod model;

use reqwest::cookie::Jar;
use reqwest::Url;
use std::sync::Arc;

pub struct Client {
    cli: reqwest::Client,
    jar: Arc<Jar>,
    base_url: Url,
}
