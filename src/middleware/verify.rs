use std::{future::{ready, Ready}};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, body::EitherBody
};

use futures_util::future::{LocalBoxFuture};
use serde::Serialize;

#[derive(Serialize)]
struct VerifyResponse {
    message: String
}

pub struct VerifyToken;

impl<S, B> Transform<S, ServiceRequest> for VerifyToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifyTokenMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifyTokenMiddleware { service }))
    }
}

pub struct VerifyTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for VerifyTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ms_token = req.headers().get("ms-token");
        if ms_token.is_some() {
            let response = self.service.call(req);
            return Box::pin(async move {
                response.await.map(ServiceResponse::map_into_left_body)
            });
        }

        let (request, _) = req.into_parts();

        let response = HttpResponse::Unauthorized()
            .json(VerifyResponse {
                message: "暂无权限访问".to_string(),
            })
            .map_into_right_body();
    
        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });

    }
}