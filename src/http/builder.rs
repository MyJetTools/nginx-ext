use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result.register_get_action(Arc::new(
        crate::http::controllers::client_certs::GetListOfRevokedCertificatesAction::new(
            app.clone(),
        ),
    ));

    result.register_post_action(Arc::new(
        crate::http::controllers::client_certs::GenerateCertificateAction::new(app.clone()),
    ));

    result.register_delete_action(Arc::new(
        crate::http::controllers::client_certs::RevokeCertificateAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::client_certs::DownloadCertAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::client_certs::DownloadPemCertificateAction::new(app.clone()),
    ));

    //CA Controller

    result.register_post_action(Arc::new(crate::http::controllers::ca::CheckCaAction::new(
        app.clone(),
    )));

    result.register_post_action(Arc::new(
        crate::http::controllers::ca::GenerateCaAction::new(app.clone()),
    ));

    result.register_post_action(Arc::new(crate::http::controllers::ca::ImportCaAction::new(
        app.clone(),
    )));

    result.register_get_action(Arc::new(
        crate::http::controllers::ca::DownloadCertAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::ca::DownloadRevokedAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::ca::DownloadPrivateKeyAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::ca::GetListOfCaAction::new(app.clone()),
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

    //Controller CaAccessList

    result.register_post_action(Arc::new(
        crate::http::controllers::client_certs_accesses::InsertAccessListIfNotExistsAction::new(
            app.clone(),
        ),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::client_certs_accesses::GetAccessListAction::new(app.clone()),
    ));

    result.register_post_action(Arc::new(
        crate::http::controllers::client_certs_accesses::AddUserAction::new(app.clone()),
    ));

    result
}
