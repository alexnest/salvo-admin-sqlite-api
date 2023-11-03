use crate::{
    api::sys::user::dao::sel, get_pool, global::cst::CURRENT_USER, 
    util::jwt::Claims, Res,
};
use salvo::{
    async_trait,
    http::{Request, Response},
    Depot, FlowCtrl, Handler,
};

/// Auth middleware
pub struct Auth;
#[async_trait]
impl Handler for Auth {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let jwt: Option<String> = req.header("Authorization");

        if let Some(jwt) = jwt {
            // handle the situation when the header has Authorization
            tracing::info!("jwt: {}", jwt);
            let claims = Claims::decode(jwt.as_str());

            match claims {
                Ok(claims) => {
                    tracing::info!("claims: {:?}", claims.sub);
                    let result = sel(get_pool(), &claims.sub).await;
                    let user = result.expect("occurs error when get user").expect("user not found");
                    depot.insert(CURRENT_USER, user);

                    ctrl.call_next(req, depot, res).await;
                }
                Err(e) => {
                    Res::fail::<()>().msg(e.to_string().as_str()).render(res);
                    ctrl.skip_rest();
                }
            }
        } else {
            // Authorization not found in the header
            Res::fail::<()>().msg("jwt notFound").render(res);
            ctrl.skip_rest();
        }
    }
}
