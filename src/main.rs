use base64::Engine;
use fastly::http::StatusCode;
use fastly::{Error, Request, Response};
use std::time::Instant;

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
    let start = Instant::now();
    let mut cur = std::io::Cursor::new(vec![]);
    let answer = tiny_captcha::gif(&mut cur);
    let image = cur.into_inner();
    let image_b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(&image);
    let elapsed_us = start.elapsed().as_micros();
    println!("Generated CAPTCHA in: {elapsed_us}");
    let body = format!(
        r#"<!DOCTYPE html>
<html>
  <head>
    <title>CAPTCHA test</title>
    <link rel="icon" href="data:,"></link>
  </head>
  <body>
    <img src="data:image/gif;base64,{image_b64}">
    <p>Answer: {answer}</p>
    <p>Generated in {elapsed_us} µs</p>
  </body>
</html>
"#
    );
    Ok(Response::from_status(StatusCode::OK).with_body_text_html(&body))
}
