use actix_web::{error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use futures::future::{Ready, ready};
use uuid::Uuid;

use crate::utils::jwt::validate_jwt_token;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub is_admin: bool,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        if let Some(auth_value) = auth_header {
            if let Ok(auth_str) = auth_value.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    
                    match validate_jwt_token(token) {
                        Ok(claims) => {
                            let user = AuthenticatedUser {
                                id: claims.user_id,
                                email: claims.email,
                                is_admin: claims.is_admin,
                            };
                            return ready(Ok(user));
                        }
                        Err(_) => {
                            return ready(Err(ErrorUnauthorized("Invalid token")));
                        }
                    }
                }
            }
        }
        
        ready(Err(ErrorUnauthorized("Missing or invalid authorization header")))
    }
}
