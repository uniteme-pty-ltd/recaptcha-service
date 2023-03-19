use super::*;

// Documentation: https://developers.google.com/recaptcha/docs/verify

#[derive(Deserialize, Debug)]
pub struct Request {
    pub secret: String,
    pub hostname: Option<String>,
    pub token: String,
    pub ip: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Response {
    score: f32,
    action: String,
    timestamp: time::PrimitiveDateTime,
}

#[derive(Serialize, Debug)]
struct RemoteRequest {
    secret: String,
    response: String,
    remoteip: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct RemoteResponse {
    success: bool,
    score: Option<f32>,
    action: Option<String>,
    #[serde(rename(deserialize = "challenge_ts"))]
    timestamp: Option<time::PrimitiveDateTime>,
    hostname: Option<String>,
    #[serde(rename(deserialize = "error-codes"))]
    errors: Option<Vec<String>>,
}

#[post("")]
pub async fn route(req: web::Json<Request>) -> Result<impl Responder, impl ResponseError> {
    let req = req.into_inner();

    let client = reqwest::Client::new();

    let remote = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&RemoteRequest {
            secret: req.secret,
            response: req.token,
            remoteip: req.ip,
        })
        .send()
        .await
        .unwrap()
        .json::<RemoteResponse>()
        .await
        .unwrap();

    match remote.success {
        false => {
            let errors = remote.errors;

            if errors.is_some()
                && errors.as_ref().unwrap().len() == 1
                && (errors.as_ref().unwrap().first().unwrap() == "invalid-input-response"
                    || errors.unwrap().first().unwrap() == "timeout-or-duplicate")
            {
                return Err(ErrorCode::Unauthorised);
            }
            Err(ErrorCode::BadRequest)
        }
        true => {
            if req.hostname.is_some()
                && (remote.hostname.is_none() || remote.hostname.unwrap() != req.hostname.unwrap())
            {
                return Err(ErrorCode::Unauthorised);
            }

            Ok(web::Json(Response {
                score: remote.score.unwrap(),
                action: remote.action.unwrap(),
                timestamp: remote.timestamp.unwrap(),
            }))
        }
    }
}
