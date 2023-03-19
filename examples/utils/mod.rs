use recaptcha_client::*;

pub fn client() -> Client {
    let port = std::env::var("PORT")
        .expect("Envrionment variable \"PORT\" not set")
        .parse::<u16>()
        .expect("Environment variable \"PORT\" must be of type u16");

    let host: String = format!("http://127.0.0.1:{}", port);

    Client::new(host)
}
