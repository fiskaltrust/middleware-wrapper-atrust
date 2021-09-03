#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#[macro_use]
pub mod helpers;

pub mod atrustapi;

mod client;
mod config;
pub mod idesscd;
mod logging;
