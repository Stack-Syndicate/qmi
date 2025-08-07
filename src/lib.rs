use hecs::World;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Sender, Receiver};
use tokio::runtime::Runtime as Tokio;

enum IoRequest {
    FetchUrl(String, Sender<String>),
}

pub struct QMI {
    world_handles: Vec<JoinHandle<()>>,
    io_tx: Sender<IoRequest>
}
impl QMI {
    pub fn new(world_count: usize) -> Self {
        let (io_tx, io_rx): (Sender<IoRequest>, Receiver<IoRequest>) = channel();
        thread::spawn(move || {
            let rt = Tokio::new().unwrap();
            rt.block_on(async move {
                while let Ok(req) = io_rx.recv() {
                    match req {
                        IoRequest::FetchUrl(url, resp_tx) => {
                            // Replace this with actual async HTTP fetch (e.g. reqwest)
                            let response = format!("Fetched: {}", url);
                            let _ = resp_tx.send(response);
                        }
                    }
                }
            });
        });
        let mut handles = Vec::with_capacity(world_count);
        for i in 0..world_count {
            handles.push(thread::spawn(move || {
                let mut world = World::new();
            }));
        }
        Self { world_handles: handles, io_tx: io_tx }
    }
}