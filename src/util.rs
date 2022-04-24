
use std::{sync::mpsc};
use anyhow::{Context, Result, Ok, Error};


pub async fn block_until_sigint() -> Result<(), Error> {
    let (ctrlc_send, ctrlc_oneshot) = mpsc::channel();
    ctrlc::set_handler(move || {
        log::warn!("Got interrupt, shutting down...");
        ctrlc_send.send(()).expect("Error sending ctrl-c message");
    }).context("Error setting Ctrl-C handler")?;

    ctrlc_oneshot.recv().context("Could not receive from channel.")?;

    Ok(())
}
