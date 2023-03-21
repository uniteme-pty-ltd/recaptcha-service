use recaptcha_client::*;

pub fn client() -> Client {
    let port = std::env::var("PORT")
        .expect("Envrionment variable \"PORT\" not set")
        .parse::<u16>()
        .expect("Environment variable \"PORT\" must be of type u16");

    let secret = std::env::var("RECAPTCHA_SECRET")
        .expect("Envrionment variable \"RECAPTCHA_SECRET\" not set");

    let client_host = match std::env::var("CLIENT_HOST") {
        Err(_) => None,
        Ok(host) => Some(host),
    };

    let host: String = format!("http://127.0.0.1:{}", port);

    Client::new(host, secret, client_host)
}
