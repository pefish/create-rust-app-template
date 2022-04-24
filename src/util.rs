
use std::sync::mpsc;



pub async fn block_until_sigint() {
    let (ctrlc_send, ctrlc_oneshot) = mpsc::channel();
    ctrlc::set_handler(move || {
        log::warn!("Got interrupt, shutting down...");
        ctrlc_send.send(()).expect("Error sending ctrl-c message");
    })
    .expect("Error setting Ctrl-C handler");

    ctrlc_oneshot.recv().expect("Could not receive from channel.");
}
