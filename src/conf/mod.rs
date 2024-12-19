#[allow(clippy::module_inception)]
mod conf;
pub use conf::Config;
mod old;
mod scope;
mod toml;
mod type_;
