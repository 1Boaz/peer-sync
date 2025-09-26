use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, web, Error};

/// Middleware function to validate the authorization header against the expected passkey.
///
/// # Arguments
/// * `req` - The incoming service request
/// * `next` - The next middleware or handler in the chain
///
/// # Returns
/// * `Result<ServiceResponse<impl MessageBody>, Error>` - Proceeds to the next middleware/handler if authorized,
///                                                      or returns an Unauthorized error if validation fails
///
/// # Errors
/// * Returns `ErrorUnauthorized` if the Authorization header is missing or doesn't match the expected passkey
pub async fn validate(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let passkey = req
        .app_data::<web::Data<String>>()
        .map(|data| data.as_str())
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Passkey not configured"))?;

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str == passkey {
                return next.call(req).await;
            }
        }
    }

    Err(actix_web::error::ErrorUnauthorized("Missing Or Invalid Token"))
}
