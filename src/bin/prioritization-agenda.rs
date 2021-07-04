use triagebot::{agenda, logger};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv::dotenv().ok();
    logger::init();

    let agenda = agenda::prioritization();

    print!("{}", agenda.call().await);
}
