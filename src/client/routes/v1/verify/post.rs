use super::*;

#[derive(Serialize)]
pub struct Request<'a> {
    secret: &'a String,
    hostname: &'a Option<String>,
    token: String,
    ip: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub score: f32,
    pub action: String,
    pub timestamp: time::PrimitiveDateTime,
}

impl Client {
    // Returns ID of created user
    pub async fn verify(
        &self,
        token: String,
        ip: Option<String>,
    ) -> Result<Option<Response>, Box<dyn std::error::Error>> {
        let res = req()
            .post(self.endpoint("/v1/verify", vec![]))
            .json(&Request {
                secret: &self.secret,
                hostname: &self.hostname,
                token,
                ip,
            })
            .send()
            .await?;

        match res.status() {
            StatusCode::UNAUTHORIZED => Ok(None),
            StatusCode::OK => Ok(Some(res.json::<Response>().await?)),
            _ => Err("Unknown Response Status Code".into()),
        }
    }
}
