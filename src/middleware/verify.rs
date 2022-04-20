use std::future::{ready, Ready};

use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{http, Error, HttpResponse, web};
use futures_util::future::LocalBoxFuture;
use serde_json::json;

use crate::config::app_data::AppGlobalData;

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

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let ms_token = request.headers().get("ms-token");
      
        let data = request.app_data::<web::Data<AppGlobalData>>();
        if ms_token.is_some() || (
            data.is_some() &&
            data.unwrap().config.ignore_matchers.contains(&request.path().to_string())
        ) {
            let res = self.service.call(request);
            return Box::pin(async move {
                Ok(res.await?.map_into_left_body())
            });
        }
        
        let (request, _pl) = request.into_parts();

        let response = HttpResponse::Unauthorized()
            .json(json!({
                "message": "There is currently no permission to access this API",
            }))
            .map_into_right_body();

        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });

      
    }
}