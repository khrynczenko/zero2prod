use serde::ser::{Serialize, SerializeStruct, Serializer};

use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

impl Serialize for FormData {
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> {
        let mut ser_struct = serializer.serialize_struct("FormData", 2)?;
        ser_struct.serialize_field("name", &self.name)?;
        ser_struct.serialize_field("email", &self.email)?;
        ser_struct.end()
    }
}

pub async fn subscribe(_from: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
