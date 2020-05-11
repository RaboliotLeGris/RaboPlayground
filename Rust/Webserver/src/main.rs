#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;

use std::io;
use std::path::Path;

use rocket::Data;
use rocket::response::{NamedFile, Debug, content::Json};
use rocket::http::ContentType;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition, FileField, TextField, RawField, MultipartFormDataError};
use std::io::{Error, Write};
use std::fs::File;

#[get("/<filename>")]
fn get_img(filename: String) -> Result<NamedFile, io::Error> {
    let path = Path::new("assets/").join(filename);
    NamedFile::open(path)
}

#[post("/upload", data = "<data>")]
fn post_img(content_type: &ContentType, data: Data) -> Result<JsonValue, Debug<io::Error>> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(
        MultipartFormDataField::raw("file").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap(),
    );

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return Err(Debug::from(Error::new(std::io::ErrorKind::InvalidInput, "Data too large")));
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return Err(Debug::from(Error::new(std::io::ErrorKind::InvalidInput, "Data not an image")));
                }
                _ => panic!("{:?}", err),
            }
        }
    };

    let image = multipart_form_data.raw.remove("file");
    match image {
        Some(image) => {
            match image {
                RawField::Single(raw) => {
                    let data = raw.raw;

                    let mut file = File::create("uploaded.jpg")?;
                    file.write_all(&data)?;
                }
                RawField::Multiple(_) => unreachable!(),
            }
        }
        None => {
            panic!("Failed to match payload")
        }
    }

    Ok(json!({ "status": "ok" }))
}

fn main() {
    rocket::ignite()
        .mount("/i", routes![get_img])
        .mount("/api", routes![post_img])
        .launch();
}