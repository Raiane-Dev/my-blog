use crate::entities::io::input::auth_entity::AuthenticateRequest;
use crate::entities::io::output::api_entity::{NetworkResponse, Response, ResponseBody};
use crate::entities::schemas::user_schema::UserSchema;
use crate::utils::jwt::{create_jwt, decode_jwt, Claims, JWT};
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::serde_json;
use rocket::serde::json::Json;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => todo!(),
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => todo!()
            },
        }
    }
}

#[post("/login", format = "application/json", data = "<input>")]
pub async fn login(input: Json<AuthenticateRequest>) -> Result<String, NetworkResponse> {
    let response: Response;

    let user = match crate::model::user_model::select_one(
        "email = $1 AND password = $2".to_string(),
        vec![&input.email, &input.plain_password],
    )
    .await
    {
        Ok(user) => {
            let token = create_jwt(user.id).unwrap();

            response = Response {
                body: ResponseBody::AuthToken(token),
            };
        }
        Err(error) => {
            response = Response {
                body: ResponseBody::Message("failed login".to_string()),
            };

            println!("{}", error)
        }
    };

    Ok(serde_json::to_string(&response).unwrap())
}
