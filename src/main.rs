use captcha_rust::Captcha;
use fastly::http::StatusCode;
use fastly::{Error, Request, Response};
use std::time::Instant;

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
    let start = Instant::now();
    let captcha = Captcha::new(5, 220, 120);
    let image_b64 = &captcha.base_img;
    let answer = &captcha.text;
    let elapsed_ms = start.elapsed().as_millis();
    println!("Generated CAPTCHA in: {elapsed_ms}");
    let body = format!(
        r#"<!DOCTYPE html>
<html>
  <head>
    <title>CAPTCHA test</title>
    <link rel="icon" href="data:,"></link>
  </head>
  <body>
    <img src="{image_b64}">
    <p>Answer: {answer}</p>
    <p>Generated in {elapsed_ms} ms</p>
  </body>
</html>
"#
    );
    Ok(Response::from_status(StatusCode::OK).with_body_text_html(&body))
}
