use utoipa::gen::serde_json;
use fornax::openapi::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();

    std::fs::write(
        "openapi.json",
        serde_json::to_string_pretty(&openapi).unwrap(),
    )
        .unwrap();

    println!("OpenAPI exported to openapi.json");
}