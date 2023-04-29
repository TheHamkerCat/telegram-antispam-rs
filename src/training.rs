use bayespam::classifier::Classifier;
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    text_type: String,
    text: String,
}

pub fn train_model<P: AsRef<std::path::Path>>(dataset_path: P, model_export_path: P) -> Result<(), csv::Error>{
    let mut classifier = Classifier::new();
    let csv = std::fs::read_to_string(dataset_path)?;

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Record = record?;
        
        if record.text_type == "spam" {
            classifier.train_spam(&record.text);
        } else {
            classifier.train_ham(&record.text);
        }
    }
    classifier.save(&mut std::fs::File::create(model_export_path)?, false)?;
    Ok(())
}