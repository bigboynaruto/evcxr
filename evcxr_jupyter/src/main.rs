// Copyright 2018 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate evcxr;
#[macro_use]
extern crate json;
#[macro_use]
extern crate failure;
extern crate chrono;
extern crate colored;
extern crate dirs;
extern crate hex;
extern crate hmac;
extern crate sha2;
extern crate uuid;
extern crate zmq;

mod connection;
mod control_file;
mod core;
mod install;
mod jupyter_message;

use failure::Error;

fn run(control_file_name: &str) -> Result<(), Error> {
    let config = control_file::Control::parse_file(&control_file_name)?;
    let server = core::Server::start(&config)?;
    server.wait_for_shutdown();
    Ok(())
}

fn main() -> Result<(), Error> {
    evcxr::runtime_hook();
    let mut args = std::env::args();
    let bin = args.next().unwrap();
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "--control_file" => {
                return run(&args
                    .next()
                    .ok_or_else(|| format_err!("Missing control file"))?);
            }
            "--install" => return install::install(),
            "--uninstall" => return install::uninstall(),
            "--help" => {}
            x => bail!("Unrecognised option {}", x),
        }
    }
    println!("To install, run:\n  {} --install", bin);
    println!("To uninstall, run:\n  {} --uninstall", bin);
    Ok(())
}
