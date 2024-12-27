use std::task::Poll;
use actix_web::dev::Service;
use actix_web::dev::Transform;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, Handler, HttpMessage};
use actix_web::{Result};
use future::Ready;
use futures_util::future::{self, LocalBoxFuture};
use futures_util::Sink;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct JwtMiddleware {
    pub(crate) secret: String,
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(JwtMiddlewareMiddleware {
            service,
            secret: self.secret.clone(),
        })
    }
}

pub struct JwtMiddlewareMiddleware<S> {
    service: S,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), actix_web::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization").cloned();
        let secret = self.secret.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            if let Some(auth_value) = auth_header {
                if let Ok(auth_str) = auth_value.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..]; // Remove "Bearer "
                        let validation = Validation::new(Algorithm::HS256);
                        let decoding_key = DecodingKey::from_secret(secret.as_ref());

                        if let Ok(_token_data) = decode::<Claims>(token, &decoding_key, &validation) {
                            return fut.await;
                        }
                    }
                }
            }

            Err(actix_web::error::ErrorUnauthorized("Invalid or missing token"))
        })
    }
}
