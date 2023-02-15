/*A libaray for translating a given text

Formula for using Github copilot:
1. Write a comment that describes what you want to do with a function
2. Format code with cargo fmt
3. Lint your code with cargo clippy

*/
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use rust_bert::pipelines::conversation::{ConversationModel, ConversationManager};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

//build a function that translates a given text
pub fn translate_text(text: Vec<String>) -> anyhow::Result<()> {
    
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::Spanish])
        .create_model()?;
    println!("Translation: ");

    for sentence in text{
        //pass in the text to the model
        let output = model.translate(&[sentence], None, Language::Spanish)?;
        let mut counter = 0;
        for result in output {
            println!("{}", result);
        }
    }
    Ok(())
}

pub fn dialogue(text: String) -> String {
    let conversation_model = ConversationModel::new(Default::default()).unwrap();
    let mut conversation_manager = ConversationManager::new();

    let conversation_id = conversation_manager.create(&text);
    let output = conversation_model.generate_responses(&mut conversation_manager);
    let response = output.get(&conversation_id).unwrap();
    response.to_string()
}