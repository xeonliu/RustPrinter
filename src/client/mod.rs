mod basic;
pub mod consts;
mod model;

use std::rc::Rc;
use std::sync::{Arc, RwLock};
use reqwest::cookie::Jar;
use reqwest::Url;

pub struct Client {
    cli: reqwest::Client,
    jar: Arc<Jar>,
    base_url: Url,
}