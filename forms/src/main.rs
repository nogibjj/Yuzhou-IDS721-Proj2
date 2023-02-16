#[macro_use] extern crate rocket;

use rocket::time::Date;
use rocket::http::{Status, ContentType};
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use rocket::fs::{FileServer, TempFile, relative};

use rocket_dyn_templates::Template;

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

// NOTE: We use `Contextual` here because we want to collect all submitted form
// fields to re-render forms with submitted values on error. If you have no such
// need, do not use `Contextual`. Use the equivalent of `Form<Submit<'_>>`.
#[post("/", data = "<form>")]
async fn submit<'r>(form: Form<Contextual<'r, Submit<'r>>>) -> (Status, Template) {
    let template = match form.value {
        Some(ref submission) => {
            println!("submission: {:#?}", submission.submission);
            print_type_of(&submission.submission);

            // let translations = translate_text(vec![String::from(submission.submission)]).await.unwrap().text().await.unwrap();
            let response = dialogue(String::from(submission.submission));
            Template::render("success", &response)
            // Template::render("success", &translations)
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


use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use rust_bert::pipelines::conversation::{ConversationModel, ConversationManager};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

//build a function that translates a given text
async fn translate_text(text: Vec<String>) -> anyhow::Result<(Vec<String>)> {
    
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::Spanish])
        .create_model()?;

    let mut answer = Vec::<String>::new();

    for sentence in text{
        //pass in the text to the model
        let output = model.translate(&[sentence], None, Language::Spanish)?;
        let mut counter = 0;
        for result in output {
            let string = String::from(result.clone());
            answer.push(string);
            // print_type_of(&(result.to_string()));
            // println!("{}", result);
        }
    }

    // println!("In answer: ");
    // for i in answer{
    //     println!("{}", i);
    // }

    Ok((answer))
}

pub fn dialogue(text: String) -> String {
    let conversation_model = ConversationModel::new(Default::default()).unwrap();
    let mut conversation_manager = ConversationManager::new();

    let conversation_id = conversation_manager.create(&text);
    let output = conversation_model.generate_responses(&mut conversation_manager);
    let response = output.get(&conversation_id).unwrap();
    response.to_string()
}