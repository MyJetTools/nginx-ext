use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result.register_get_action(Arc::new(
        crate::http::controllers::certificates::GetListOfRevokedCertificatesAction::new(
            app.clone(),
        ),
    ));

    result.register_post_action(Arc::new(
        crate::http::controllers::certificates::GenerateCertificateAction::new(app.clone()),
    ));

    result.register_delete_action(Arc::new(
        crate::http::controllers::certificates::RevokeCertificateAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::certificates::DownloadCertAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::certificates::DownloadPemCertificateAction::new(app.clone()),
    ));

    result.register_post_action(Arc::new(
        crate::http::controllers::ca::GenerateCaAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::ca::DownloadCertAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::ca::DownloadRevokedAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::ca::DownloadPrivateKeyAction::new(app.clone()),
    ));

    //NGINX Upstream Controller
    result.register_post_action(Arc::new(
        crate::http::controllers::nginx_upstream::InsertOrReplaceUpstreamAction::new(app.clone()),
    ));

    result.register_delete_action(Arc::new(
        crate::http::controllers::nginx_upstream::DeleteUpstreamAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::nginx_upstream::GetNginxConfigurationAction::new(app.clone()),
    ));

    // NGINX HTTP Controller

    result.register_post_action(Arc::new(
        crate::http::controllers::nginx_http::InsertOrReplaceAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::nginx_http::GetNginxConfigurationAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::nginx_http::GetAction::new(app.clone()),
    ));

    result.register_delete_action(Arc::new(
        crate::http::controllers::nginx_http::DeleteHttpConfigAction::new(app.clone()),
    ));

    // NGINX Controller
    result.register_post_action(Arc::new(
        crate::http::controllers::nginx::ReloadAction::new(app.clone()),
    ));

    result.register_post_action(Arc::new(
        crate::http::controllers::nginx::GenerateConfigFileAction::new(app.clone()),
    ));

    // NGINX Templates Controller

    result.register_post_action(Arc::new(
        crate::http::controllers::nginx_templates::InsertOrReplaceAction::new(app.clone()),
    ));

    result.register_delete_action(Arc::new(
        crate::http::controllers::nginx_templates::DeleteTemplateAction::new(app.clone()),
    ));

    // SSL Controller

    result.register_post_action(Arc::new(
        crate::http::controllers::ssl::UploadSslCertificateAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::ssl::GetListOfCertsAction::new(app.clone()),
    ));

    result
}
