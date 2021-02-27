use message_handling::run;

mod message_handling;
mod states;
mod transitions;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("bot started");

    run().await;
}
