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

    result.register_post_action(Arc::new(
        crate::http::controllers::nginx_upstream::InsertOrReplaceUpstreamAction::new(app.clone()),
    ));

    result.register_delete_action(Arc::new(
        crate::http::controllers::nginx_upstream::DeleteUpstreamAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        crate::http::controllers::nginx_upstream::GetNginxConfigurationAction::new(app.clone()),
    ));

    result.register_post_action(Arc::new(crate::http::controllers::nginx::ReloadAction));
    result
}
