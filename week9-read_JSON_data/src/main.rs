use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
        "article": "The Rust Programming Language",
        "author": "Steve Klabnik and Carol Nichols",
        "paragraphs": [
            {
                "name": "Introduction"
            },
            {
                "name": "What is Rust?"
            },
            {
                "name": "Installation"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!("\n\n The name of the second paragraph is: {}", parsed.paragraphs[1].name);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed
}