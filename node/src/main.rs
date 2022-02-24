//! Liberland Node CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
// mod identity_rpc;
// mod min_interior_rpc;
mod referendum_rpc;
mod rpc;

fn main() -> sc_cli::Result<()> {
    command::run()
}
