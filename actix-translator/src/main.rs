#[macro_use] extern crate rocket;
mod lib;
use rocket::time::Date;
use rocket::http::{Status, ContentType};
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use serde::Serialize;
use serde::Deserialize;
use rocket::fs::{FileServer, TempFile, relative};
use rust_bert::pipelines::translation::{
    Language, TranslationConfig, TranslationModel, TranslationModelBuilder,
};
use rocket_dyn_templates::Template;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub translation: String,
}

#[derive(Debug, FromForm)]
#[allow(dead_code)]
struct Submit<'v> {
    #[field(validate = len(1..=250))]
    r#submission: &'v str,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &Context::default())
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
// NOTE: We use `Contextual` here because we want to collect all submitted form
// fields to re-render forms with submitted values on error. If you have no such
// need, do not use `Contextual`. Use the equivalent of `Form<Submit<'_>>`.
#[post("/", data = "<form>")]
async fn submit<'r>(form: Form<Contextual<'r, Submit<'r>>>) -> (Status, Template) {
    let translation_model = libs::init_translation_model();

    let template = match form.value {
        Some(ref submission) => {
            println!("submission: {:#?}", submission.submission);

            let translations = translation_model.translate(&[submission.submission], None, Language::Spanish).unwrap();
            let full_translations = translations.join("\n");
            
            let response_json = &GenericResponse {
                status: "success".to_string(),
                translation: full_translations.to_string(),
            };
            Template::render("success", response_json)

            
            // let response = dialogue(String::from(submission.submission));
            // Template::render("success", &response)
        }
        None => Template::render("index", &form.context),
    };

    (form.context.status(), template)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
}