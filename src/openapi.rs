use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "http://127.0.0.1:8787", description = "Local"),
        (url = "https://fornax.com", description = "Production")
    ),
    paths(
        crate::auth::handler::login,
        crate::auth::handler::send_code,
        crate::router::root,
    ),
    components(
        schemas(
            crate::auth::dto::LoginReq,
            crate::auth::dto::LoginResp,
            crate::auth::dto::SendCodeReq,
            crate::auth::dto::SendCodeResp,
        )
    ),
    tags(
        (name = "Auth", description = "认证接口")
    )
)]
pub struct ApiDoc;