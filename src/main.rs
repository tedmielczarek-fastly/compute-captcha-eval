use base64::Engine;
use captcha_a::{CaptchaBuilder, Font};
use fastly::http::StatusCode;
use fastly::{Error, Request, Response};
use std::time::Instant;

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
    let start = Instant::now();
    let fonts = vec![Font::try_from_bytes(include_bytes!("../NotoSans-Regular.ttf")).unwrap()];
    let builder = CaptchaBuilder {
        //custom attribute
        width: 120,
        height: 40,
        length: 4,
        fonts: &fonts,
        //default attribute
        ..Default::default()
    };
    let captcha = builder.build().unwrap();
    let image = &captcha.raw_data;
    let answer = &captcha.phrase;
    let elapsed_ms = start.elapsed().as_millis();
    println!("Generated CAPTCHA in: {elapsed_ms}");
    let image_b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(image);
    let body = format!(
        r#"<!DOCTYPE html>
<html>
  <head>
    <title>CAPTCHA test</title>
    <link rel="icon" href="data:,"></link>
  </head>
  <body>
    <img src="data:image/png;base64,{image_b64}">
    <p>Answer: {answer}</p>
    <p>Generated in {elapsed_ms} ms</p>
  </body>
</html>
"#
    );
    Ok(Response::from_status(StatusCode::OK).with_body_text_html(&body))
}
