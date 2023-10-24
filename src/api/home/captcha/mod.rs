use captcha_rs::CaptchaBuilder;

use rand::Rng;
use salvo::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::{cache, AppResult, Res};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CaptchaRes {
    img_base64: String,
    uuid: String,
}

#[allow(dead_code)]
pub fn create_captcha() -> (i32, String) {
    // generate a random math problem
    let num1 = rand::thread_rng().gen_range(5..=10);
    let num2 = rand::thread_rng().gen_range(1..=5);
    let operator = ['+', '-', '*'][rand::thread_rng().gen_range(0..3)];

    let question = format!("{} {} {} = ?", num1, operator, num2);
    tracing::info!("question: {}", question);

    // calculate the answer
    let answer = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        _ => 0, // 这个分支不会被执行，只是为了处理未知操作符
    };

    let captcha = CaptchaBuilder::new()
        .text(question)
        .width(200)
        .height(70)
        .dark_mode(true)
        .complexity(10) // min: 1, max: 10
        .compression(40) // min: 1, max: 99
        .build();
    (answer, captcha.to_base64())
}

#[handler]
pub async fn get_captcha(res: &mut Response) -> AppResult<()> {
    let (answer, base64) = create_captcha();
    let uuid = Uuid::new_v4().to_string();

    tracing::info!("uuid: {}", uuid);

    let data = CaptchaRes {
        img_base64: base64,
        uuid: uuid.clone(),
    };

    cache::insert(String::from(uuid), answer).await;

    Res::suc::<CaptchaRes>().data(data).render(res);
    Ok(())
}
