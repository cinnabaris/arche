use rocket::http::Status;

#[error(404)]
fn not_found() -> &'static str {
    Status::NotFound.reason
}

#[error(400)]
fn bad_request() -> &'static str {
    Status::BadRequest.reason
}

#[error(403)]
fn forbidden() -> &'static str {
    Status::Forbidden.reason
}

#[error(500)]
fn internal_server() -> &'static str {
    Status::InternalServerError.reason
}
