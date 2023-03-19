use utils::*;

mod utils;

#[tokio::main]
async fn main() {
    let result = client()
        .verify(
            std::env::var("RECAPTCHA_SECRET")
                .expect("RECAPTCHA_SECRET environment variable needs to be set"),
            None,
            std::env::var("RECAPTCHA_TOKEN")
                .expect("RECAPTCHA_TOKEN environment variable needs to be set"),
            None,
        )
        .await;

    println!("{:?}", result);
}
