use std::{thread, time, sync::mpsc};

#[derive(PartialEq)]
enum CMD {
    PLAY,
    STOP,
}

const TURN_DURATION: time::Duration = time::Duration::from_secs(1);

// Two threads are passing a 'ball' via a channels, printing either 'ping' or 'pong' respective of thread.
fn main() {
    let (ping_tx, ping_rx) = mpsc::sync_channel::<CMD>(1);
    let (pong_tx, pong_rx) = mpsc::sync_channel::<CMD>(1);

    let ping_tx_clone = ping_tx.clone();
    let pong_tx_clone = pong_tx.clone();

    ping_tx.send(CMD::PLAY).unwrap();

    let join_handles = [
        thread::spawn(||ping_pong(pong_tx, ping_rx, "ping")),
        thread::spawn(||ping_pong(ping_tx, pong_rx, "pong"))
    ];

    thread::sleep(TURN_DURATION * 5);

    ping_tx_clone.send(CMD::STOP).unwrap();
    pong_tx_clone.send(CMD::STOP).unwrap();

    for handle in join_handles {
        handle.join().unwrap();
    }
}

fn ping_pong(tx: mpsc::SyncSender<CMD>, rx: mpsc::Receiver<CMD>, side: &str) {
    loop {
        match rx.recv().unwrap() {
            CMD::PLAY => {
                println!("{}", side);
                thread::sleep(TURN_DURATION);
                tx.send(CMD::PLAY).unwrap();
            }
            CMD::STOP => {
                thread::sleep(TURN_DURATION * 2); // postpone closing the channel to avoid SendError on other side
                return;
            }
        }
    }
}
