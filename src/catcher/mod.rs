use crate::res::result::Res;
use salvo::prelude::*;

#[handler]
pub fn handle_invalid_http_code(
    &self,
    _req: &Request,
    _depot: &Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    if let Some(status_code) = res.status_code {
        let code = status_code.as_u16();
        if code == 404 {
            res.render(Json(Res::fail::<()>().code(404).msg("Not Found")));
            ctrl.skip_rest();
        }
        if code == 500 {
            res.render(Json(
                Res::fail::<()>().code(500).msg("Internal Server Error"),
            ));
            ctrl.skip_rest();
        }
    }
}
