use rocket::serde::{Deserialize, json::Json, json::json, json::Value};

use bayespam::classifier;

// To avoid creating a new instance of the classifier on each request
lazy_static! {
    static ref CLASSIFIER: classifier::Classifier = {
        classifier::Classifier::new_from_pre_trained(
            &mut std::fs::File::open("model.json").unwrap()
        ).unwrap()
    };
}
static SPAM_PROB_THRESHOLD: u8 = 70;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    text: std::borrow::Cow<'r, str>,
}

#[post("/", data = "<task>")]
fn spam_check(task: Json<Task<'_>>) -> Value {
    // let is_spam = CLASSIFIER.identify(task.text);
    let score = (CLASSIFIER.score(task.text.as_ref()) *100.0) as u8;

    let mut spam = score;
    let mut ham = 100 - spam;

    // Short messages are mostly ham
    if spam > ham && task.text.len() < 20 {
        spam -= 25;
        ham += 25;
    }

    json!(
        {
            "is_spam": score > SPAM_PROB_THRESHOLD,
            "spam_probability": score,
            "spam": spam,
            "ham": ham,
            "profanity": false, // Always false, for backwards compatibility in ARQ
        }
    )
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/spam_check", routes![spam_check])
    })
}