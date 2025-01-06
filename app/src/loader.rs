use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tokio::time::Duration;

// Loader functions
pub fn start_loader(message: &str) -> Arc<AtomicBool> {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    let msg = message.to_string();

    thread::spawn(move || {
        let mut counter = 0;
        while running_clone.load(Ordering::SeqCst) {
            let animation = ["|", "/", "-", "\\"];
            print!("\r{} {}", animation[counter % 4], msg);
            std::io::stdout().flush().unwrap();
            counter += 1;
            thread::sleep(Duration::from_millis(200));
        }
        print!("\r{}{}", " ".repeat(msg.len() + 5), "\r");
        std::io::stdout().flush().unwrap();
    });

    running
}

pub fn stop_loader(loader: Arc<AtomicBool>) {
    loader.store(false, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(300));
}
