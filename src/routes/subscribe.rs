use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

impl FormData {
    pub fn validate(&self) -> bool {
        !self.name.is_empty() && self.validate_email()
    }

    fn validate_email(&self) -> bool {
        self.email.contains('@')
    }
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    if !form.validate() {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().finish()
}
