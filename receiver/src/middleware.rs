use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, web, Error};

pub async fn validate(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let passkey = req.app_data::<web::Data<String>>().unwrap().to_string();

    if let Some(auth_header) = req.headers().get("Authorization") {
        println!("{:?}", auth_header);
        if let Ok(auth_str) = auth_header.to_str() {
            println!("{}", auth_str);
            if auth_str == passkey {
                return next.call(req).await;
            }
        }
    }

    Err(actix_web::error::ErrorUnauthorized("Missing Or Invalid Token"))
}
