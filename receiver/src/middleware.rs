use crate::error::MiddlewareError;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use subtle::ConstantTimeEq;


/// Middleware function to validate the authorization header against the expected passkey.
///
/// # Arguments
/// * `req` - The incoming service request
/// * `next` - The next middleware or handler in the chain
///
/// # Returns
/// * `Result<ServiceResponse<impl MessageBody>, Error>` - Proceeds to the next middleware/handler if authorized,
///                                                      or returns an appropriate error
///
/// # Errors
/// * Returns `Unauthorized` if the Authorization header is missing or doesn't match the expected passkey
/// * Returns `Configuration` if the passkey is not properly configured
pub async fn validate(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let passkey = req
        .app_data::<web::Data<String>>()
        .ok_or_else(|| {
            let err = MiddlewareError::Configuration("Passkey not configured".into());
            actix_web::error::ErrorInternalServerError(err)
        })?.as_str();

    let auth_header = req.headers()
        .get("Authorization")
        .ok_or_else(|| {
            actix_web::error::ErrorUnauthorized(MiddlewareError::Unauthorized)
        })?;

    let auth_str = auth_header.to_str().map_err(|_| {
        actix_web::error::ErrorUnauthorized(MiddlewareError::Unauthorized)
    })?;

    if auth_str.as_bytes().ct_eq(passkey.as_bytes()).into() {
        next.call(req).await
    } else {
        Err(actix_web::error::ErrorUnauthorized(MiddlewareError::Unauthorized))
    }
}
