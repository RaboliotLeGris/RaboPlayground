#![feature(proc_macro_hygiene, decl_macro)]

extern crate nanoid;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::{Error, Write};
use std::path::Path;

use nanoid::nanoid;
use rocket::Data;
use rocket::http::ContentType;
use rocket::response::{Debug, NamedFile};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use rocket_contrib::serve::StaticFiles;
use rocket_multipart_form_data::{mime, MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions, RawField};

#[get("/<filename>")]
fn get_img(filename: String) -> Result<NamedFile, io::Error> {
    let path = Path::new("uploaded/").join(filename);
    NamedFile::open(path)
}

#[post("/upload", data = "<data>")]
fn post_img(content_type: &ContentType, data: Data) -> Result<JsonValue, Debug<io::Error>> {
    let img_field_name = "img";

    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(
        MultipartFormDataField::raw(img_field_name).content_type_by_string(Some(mime::IMAGE_STAR)).unwrap(),
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

    let image = multipart_form_data.raw.remove(img_field_name);
    match image {
        Some(image) => {
            match image {
                RawField::Single(raw) => {
                    // TODO: return template html with image & link to this image
                    let id = nanoid!(10);
                    let mut file = File::create(format!("uploaded/{}.{}", id, get_extension(&raw.file_name)))?;
                    file.write_all(&raw.raw)?;
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

fn get_extension(filename: &Option<String>) -> String {
    match filename {
        Some(s) => {
            if let Some(os_filename) = Path::new(&s).extension().and_then(OsStr::to_str) {
                String::from(os_filename)
            } else {
                String::from("bin")
            }
        }
        None => String::from("bin")
    }
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/i", routes![get_img, post_img])
        .launch();
}