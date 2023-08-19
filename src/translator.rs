use fluent_bundle::{FluentBundle, FluentResource, FluentArgs};
use unic_langid::langid;

pub fn get(language: &String, msg_id: &str, args: &FluentArgs<'_>) -> String {
    let value = match language.as_str() {
        "ru" | "ru_RU" => get_ru(msg_id, args),
        "en" | "en_US" => get_en(msg_id, args),
        _ => get_ru(msg_id, args)
    };

    value
}


fn get_ru(msg_id: &str, args: &FluentArgs<'_>) -> String {

    // 1. Crate a FluentResource

    let ftl_string = include_str!("../locales/ru_RU.ftl").to_string();

    let res = FluentResource::try_new(ftl_string)
        .expect("Failed to parse an FTL string.");


    // 2. Crate a FluentBundle

    let langid_en = langid!("en-US");
    let mut bundle = FluentBundle::new(vec![langid_en]);


    // 3. Add the resource to the bundle

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    // return bundle;

    // 4. Retrieve a FluentMessage from the bundle

    let msg = bundle.get_message(msg_id)
        .expect("Message doesn't exist.");


    // 5. Format the value of the simple message

    let mut errors = vec![];

    let pattern = msg.value()
        .expect("Message has no value.");

    let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
    
    value.to_string()
}

fn get_en(msg_id: &str, args: &FluentArgs<'_>) -> String {

    // 1. Crate a FluentResource

    let ftl_string = include_str!("../locales/en_US.ftl").to_string();

    let res = FluentResource::try_new(ftl_string)
        .expect("Failed to parse an FTL string.");


    // 2. Crate a FluentBundle

    let langid_en = langid!("en-US");
    let mut bundle = FluentBundle::new(vec![langid_en]);


    // 3. Add the resource to the bundle

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    // return bundle;

    // 4. Retrieve a FluentMessage from the bundle

    let msg = bundle.get_message(msg_id)
        .expect("Message doesn't exist.");


    // 5. Format the value of the simple message

    let mut errors = vec![];

    let pattern = msg.value()
        .expect("Message has no value.");

    let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
    
    value.to_string()
}