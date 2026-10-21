#![windows_subsystem = "windows"]
#![feature(str_as_str)]

use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
use windows_core;
use windows_reactor::*;
use std::sync::{OnceLock};
use tokio::runtime::{Builder, Handle};

use crate::components::main_window::MainWindow;

pub mod extensions;
pub mod components;
pub mod helpers;
pub mod network;

static TOKIO: OnceLock<Handle> = OnceLock::new();

pub fn tokio_handle() -> &'static Handle {
    TOKIO.get_or_init(|| {
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime.");

        let handle = runtime.handle().clone();

        std::thread::Builder::new()
            .name("tokio-runtime".into())
            .spawn(move || {
                runtime.block_on(std::future::pending::<()>());
            })
            .expect("Runtime failed to start");

        handle
    })
}

fn main() -> windows_core::Result<()> {
    unsafe { AttachConsole(ATTACH_PARENT_PROCESS); }
    App::run_component::<MainWindow>(())
}
