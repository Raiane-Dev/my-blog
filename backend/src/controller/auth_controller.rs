use crate::entities::io::input::auth_entity::AuthenticateRequest;
use crate::entities::io::output::api_entity::{NetworkResponse, Response, ResponseBody};
use crate::entities::schemas::user_schema::UserSchema;
use crate::utils::jwt::{create_jwt, decode_jwt, Claims, JWT};
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::json::serde_json;
use rocket::serde::json::Json;
use std::env;
use serde::Deserialize;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, NetworkResponse> {
        fn is_valid(token: &str) -> Result<Claims, Error> {
            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
            let validation = Validation::new(Algorithm::HS512);

            let token_data: TokenData<Claims> = decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &validation,
            )?;

            Ok(token_data.claims)
        }

        match req.cookies().get("x-access-token") {
            None => Outcome::Error((
                Status::Unauthorized,
                NetworkResponse::Unauthorized("Not authorized".to_string()),
            )),
            Some(cookie) => match is_valid(cookie.value()) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => {
                    println!("{}", err);
                    let response = Response {
                        body: ResponseBody::Message(format!(
                            "Error validating JWT token - {}",
                            err
                        )),
                    };
                    Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()),
                    ))
                }
            },
        }
    }
}

#[get("/check-auth")]
pub async fn check_auth(_auth: JWT) -> Result<String, NetworkResponse> {
    let response = Response {
        body: ResponseBody::Message("check".to_string()),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/login", format = "application/json", data = "<input>")]
pub async fn login(
    jar: &CookieJar<'_>,
    input: Json<AuthenticateRequest>,
) -> Result<String, NetworkResponse> {
    let response: Response;

    let _user = match crate::model::user_model::select_one(
        "email = $1 AND password = $2".to_string(),
        vec![&input.email, &input.plain_password],
    )
    .await
    {
        Ok(user) => {
            let token = create_jwt(user.id).unwrap();

            response = Response {
                body: ResponseBody::AuthToken(token.clone()),
            };

            jar.add(Cookie::build(("x-access-token", token)).expires(None));
        }
        Err(err) => {
            println!("{}", err);
            return Err(NetworkResponse::BadRequest(err.to_string()));
        }
    };

    Ok(serde_json::to_string(&response).unwrap())
}
