#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod constants;
mod docker_commands;
mod docker_error;
mod docker_instance;
mod docker_instance_builder;

/// Contains all useful structures.
pub mod models;

pub use docker_error::{DockerError, Result};
pub use docker_instance::DockerInstance;
pub use docker_instance_builder::DockerInstanceBuilder;
