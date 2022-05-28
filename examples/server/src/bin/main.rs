use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use server::App;
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};
use futures::executor::block_on;
use futures::future::ready;

fn render() -> String {
    let renderer = yew::ServerRenderer::<App>::new();
    let content = block_on(renderer.render());
    // ready(content);
    format!("<body>{:?}", content)
}

fn handle_http() -> bytecodec::Result<Response<String>> {
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        format!("echo: {}", render()),
    ))
}

fn handle_error(req: Request<String>) -> bytecodec::Result<Response<String>> {
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(500).unwrap(),
        ReasonPhrase::new("Failed to get resource").unwrap(),
        "Not found!".to_string(),
    ))
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(),
        Err(e) => Err(e),
    };

    // let write_buf = Ok(req).body();
    // stream.write(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("new connection at {}", port);
    let listener = //TcpStream::connect(format!("127.0.0.1:{}", port))?;
    TcpListener::bind(format!("0.0.0.0:{}", port), false)?;

    loop {
        let _ = handle_client(listener.accept(false)?.0);
    }

}
