use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/nginx/v1/reload",
    summary: "Reloads nginx",
    description: "Reloads nginx",
    controller: "Nginx",
    result:[
        {status_code: 202, description: "Ok result"},
    ]
)]
pub struct ReloadAction;

async fn handle_request(
    _action: &ReloadAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    match crate::storage::nginx::instance::reload().await {
        Ok(result) => {
            return HttpOutput::as_text(result).into_ok_result(true).into();
        }
        Err(e) => return Err(HttpFailResult::as_fatal_error(e)),
    }
}
