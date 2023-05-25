use base64::Engine;
use fastly::http::StatusCode;
use fastly::{Error, Request, Response};
use std::time::Instant;

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
    let start = Instant::now();
    //TODO: generate an actual CAPTCHA
    let image = include_bytes!("../see-no-evil.png");
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
    <p>Generated in {elapsed_ms} ms</p>
  </body>
</html>
"#
    );
    Ok(Response::from_status(StatusCode::OK).with_body_text_html(&body))
}
