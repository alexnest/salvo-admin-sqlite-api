use salvo::http::{self,Body, Request, Response};
use salvo::{async_trait, Depot, FlowCtrl, Handler};
use crate::res::result::Res;

/// MaxSize
pub struct MaxSize(pub u64);
#[async_trait]
impl Handler for MaxSize {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let size_hint = req.body().size_hint().upper();
        if let Some(upper) = size_hint {
            if upper > self.0 {
                res.status_code(http::StatusCode::PAYLOAD_TOO_LARGE);
                ctrl.skip_rest();
            } else {
                ctrl.call_next(req, depot, res).await;
            }
        } else {
            res.status_code(http::StatusCode::BAD_REQUEST);
            Res::fail::<&str>().data("body size is unknown").render(res);
            ctrl.skip_rest();
        }
    }
}

pub fn max_size(size: u64) -> MaxSize {
    MaxSize(size)
}
