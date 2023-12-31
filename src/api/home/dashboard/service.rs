use salvo::prelude::*;

use crate::{AppResult, Res};

#[handler]
pub async fn dashboard(_req: &mut Request, res: &mut Response) -> AppResult<()> {
  Res::suc::<_>().data("开源等于互助；开源需要大家一起来支持，支持的方式有很多种，比如使用、推荐、写教程、保护生态、贡献代码、回答问题、分享经验、打赏赞助等；欢迎您加入我们！").render(res);
  Ok(())
}