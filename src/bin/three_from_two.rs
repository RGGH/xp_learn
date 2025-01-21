use serde::{Serialize, Deserialize};
use serde_json::Value;

// Define the input and output structures
#[derive(Debug, Clone, Serialize, Deserialize)]  // Add Serialize here
struct Document {
    id: String,
    data: String,
}

#[derive(Debug)]
struct Embedding {
    vec: Vec<u8>,
}

#[derive(Debug)]
struct PointStruct {
    id: String,
    vector: Vec<f32>,
    payload: Value,
}

impl PointStruct {
    fn new(id: String, vector: Vec<f32>, payload: Value) -> Self {
        Self { id, vector, payload }
    }
}

fn main() {
    // Example input data
    let documents = vec![
        Document {
            id: "doc1".to_string(),
            data: "Some data for doc1".to_string(),
        },
        Document {
            id: "doc2".to_string(),
            data: "Some data for doc2".to_string(),
        },
    ];

    let embeddings = vec![
        Embedding {
            vec: vec![1, 2, 3],e rig::{
    embeddings::EmbeddingsBuilder,
    loaders::PdfFileLoader,
    providers::openai::{self, TEXT_EMBEDDING_ADA_002},
    vector
        },
        Embedding {
            vec: vec![4, 5, 6],
        },
    ];

    // Combine inputs into a Vec of tuples for demonstration
    // Creates an iterator that combines two iterators (documents and embeddings) into a single iterator of tuples
    let combined = documents.into_iter().zip(embeddings.into_iter());

    // Transform each pair of inputs into a PointStruct
    let points: Vec<PointStruct> = combined
        .map(|(doc, embedding)| {
            let vector: Vec<f32> = embedding.vec.iter().map(|&x| x as f32).collect();
            let payload = serde_json::to_value(&doc).unwrap();
            PointStruct::new(doc.id, vector, payload)
        })
        .collect();

    // Print the output
    for point in points {
        println!("{:?}", point);
    }
}

