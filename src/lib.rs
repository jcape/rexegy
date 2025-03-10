//! Unofficial Exegy Rust Bindings

pub use self::{
    error::{Error, ExegyError, Result},
    feed::{Feed, Id as FeedId},
    group::{Corporate, Country, Group, Id as GroupId},
    key::{Key, Symbol},
    timing::EventTiming,
};

pub mod container;
pub mod event;
pub mod object;
pub mod session;

mod error;
mod feed;
mod field;
mod group;
mod key;
mod line;
mod macros;
mod timing;
