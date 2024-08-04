use async_openai::{types::CreateEmbeddingRequestArgs, Client};
use elasticsearch::http::headers::{HeaderMap, HeaderValue, CONTENT_LENGTH};
use elasticsearch::{
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    indices::IndicesCreateParts,
    Elasticsearch, Error as ElasticsearchError,
};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client as ReqwestClient;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub const ES_URL: &str = "https://69bc680d7967407080cd9090e3c12a25.us-central1.gcp.cloud.es.io:443";
pub const ES_API_KEY: &str = "UWdUV0M1RUJjd1F5SmpPNHRJVlU6Ui1tenFqaUFReFc5d0k2ODJSVnBldw==";

pub(crate) fn create_client() -> Result<Elasticsearch, Box<dyn std::error::Error>> {
    let conn_pool = SingleNodeConnectionPool::new(ES_URL.parse()?);
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("ApiKey {}", ES_API_KEY))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?,
    );
    let transport = TransportBuilder::new(conn_pool).headers(headers).build()?;
    let client = Elasticsearch::new(transport);
    Ok(client)
}

pub(crate) async fn generate_embedding(
    code_snippet: &str,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let config = async_openai::config::OpenAIConfig::new().with_api_key(
        env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set")
            .as_str(),
    );
    let oai = Client::with_config(config);
    let response = oai
        .embeddings()
        .create(
            CreateEmbeddingRequestArgs::default()
                .model("text-embedding-ada-002")
                .input(code_snippet)
                .build()?,
        )
        .await?;
    Ok(response.data[0].embedding.clone())
}

async fn create_document(
    fpath: &Path,
) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let module_code = fs::read_to_string(fpath)?;
    let embedding = generate_embedding(&module_code).await?;
    Ok(HashMap::from([
        (
            "name".to_string(),
            Value::String(fpath.file_name().unwrap().to_str().unwrap().to_string()),
        ),
        ("code".to_string(), Value::String(module_code)),
        ("embedding".to_string(), json!(embedding)),
    ]))
}

pub(crate) async fn embed_library(
    fpath: &Path,
    index: &str,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut documents = Vec::new();
    let mut idx = 0;
    for entry in walkdir::WalkDir::new(fpath)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()).map(|ext| ext == "v" || ext == "vhdl" || ext == "vhd").unwrap_or(false) {
            let doc = create_document(path).await?;
            documents.push(json!({
                "index": {
                    "_index": index,
                    "_id": idx
                }
            }));
            documents.push(json!(doc));
            idx += 1;
        }
    }
    Ok(documents)
}

pub(crate) async fn create_index(
    client: &Elasticsearch,
    index_name: &str,
) -> Result<(), ElasticsearchError> {
    let body = json!({
        "settings": {
            "number_of_shards": 1,
            "number_of_replicas": 1
        },
        "mappings": {
            "properties": {
                "name": {"type": "keyword"},
                "code": {"type": "text"},
                "embedding": {
                    "type": "dense_vector",
                    "dims": 1536,
                    "index": true,
                    "similarity": "cosine"
                }
            }
        }
    });

    let body_string = serde_json::to_string(&body)?;
    let body_length = body_string.len();

    client
        .indices()
        .create(IndicesCreateParts::Index(index_name))
        .header(
            CONTENT_LENGTH,
            HeaderValue::from_str(&body_length.to_string()).unwrap(),
        )
        .body(body)
        .send()
        .await?;

    Ok(())
}

pub(crate) async fn insert_documents(
    index_name: &str,
    embedded_documents: &[Value],
) -> Result<(), ElasticsearchError> {
    let client = ReqwestClient::new();
    for chunk in embedded_documents.chunks(2) {
        if chunk.len() == 2 {
            let doc_id = chunk[0]["index"]["_id"].as_str();
            let doc_body = &chunk[1];

            let url = match doc_id {
                Some(id) => format!("{}/{}/_doc/{}", ES_URL, index_name, id),
                None => format!("{}/{}/_doc", ES_URL, index_name),
            };
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("ApiKey {}", ES_API_KEY)).unwrap(),
            );
            let request = if doc_id.is_some() {
                client.put(&url)
            } else {
                client.post(&url)
            };
            request.headers(headers).json(doc_body).send().await?;
        }
    }
    Ok(())
}

// pub(crate) async fn vector_search(
//     client: &Elasticsearch,
//     index_name: &str,
//     query_vector: Vec<f32>,
//     top_k: usize,
// ) -> Result<Vec<HashMap<String, Value>>, ElasticsearchError> {
//     let search_query = json!({
//         "size": top_k,
//         "query": {
//             "script_score": {
//                 "query": {"match_all": {}},
//                 "script": {
//                     "source": "cosineSimilarity(params.query_vector, 'embedding') + 1.0",
//                     "params": {"query_vector": query_vector}
//                 }
//             }
//         }
//     });
//
//     let response = client
//         .search(elasticsearch::SearchParts::Index(&[index_name]))
//         .body(search_query)
//         .send()
//         .await?;
//
//     let mut results = Vec::new();
//     if let Some(hits) = response.json::<Value>().await?["hits"]["hits"].as_array() {
//         for hit in hits {
//             results.push(HashMap::from([
//                 ("id".to_string(), hit["_id"].clone()),
//                 ("score".to_string(), hit["_score"].clone()),
//                 ("name".to_string(), hit["_source"]["name"].clone()),
//                 ("code".to_string(), hit["_source"]["code"].clone()),
//             ]));
//         }
//     }
//     Ok(results)
// }
