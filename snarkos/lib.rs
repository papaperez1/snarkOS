// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

#![forbid(unsafe_code)]
#![allow(clippy::module_inception)]
#![allow(clippy::suspicious_else_formatting)]
#![allow(clippy::type_complexity)]

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate tracing;

pub mod display;

pub mod cli;
pub use cli::*;

pub mod node;
pub use node::*;

pub mod updater;
pub use updater::*;

#[cfg(test)]
mod tests;

pub use snarkos_environment as environment;
// pub use snarkos_storage as storage;

#[cfg(feature = "rpc")]
pub use snarkos_rpc as rpc;

pub use snarkvm::prelude::{Address, Network};

pub mod prelude {
    pub use crate::environment::*;

    #[cfg(feature = "rpc")]
    pub use crate::rpc::*;

    pub use snarkvm::prelude::{Address, Network};
}
