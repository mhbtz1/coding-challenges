use actix::Actor;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use base64::{engine::general_purpose, Engine as _};
use std::{io::{Read, Write}, sync::Arc};
use flexi_logger::{Duplicate, Logger, FileSpec};
use log::*;


const BASE_TINY_URL: &'static str = "http://tiny.co";

#[derive(Deserialize)]
struct TinifyPayload {
    url: String
}

#[derive(Serialize)]
struct TinifyResponse {
    tinified_url: String
}


#[derive(Deserialize)]
struct DetinifyPayload {
    tinified_url: String,
}

#[derive(Serialize)]
enum Status {
    SUCCESS,
    ERROR
}

#[derive(Serialize)]
struct DetinifyResponse {
    status: Status,
    detinified_url: String   
}


#[get("/")]
async fn tget() -> impl Responder {
    HttpResponse::Ok().body("Test GET endpoint")
}

#[post("/tinify")]
async fn tinify(web::Form(form): web::Form<TinifyPayload>) -> impl Responder {
    let val: String = form.url.clone();
    let splitUrl = val.split('/').into_iter().collect::<Vec<_>>();

    info!("base_url: {:?}", &val);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());

    let mut n_split = splitUrl[0].to_string();
    let slash_pos = val.match_indices('/')
           .nth(2)  // 3rd slash
           .map(|(idx, _)| idx)
           .unwrap_or_else(|| {
               info!("Less than three slashes found in {:?}", val);
               usize::MAX    
           });

    if slash_pos != usize::MAX  {
        if n_split.starts_with("http") {
            n_split = n_split["http://".len()..slash_pos].to_string();
        } else {
            n_split = n_split[0..slash_pos].to_string()
        }
    } else if n_split.starts_with("http") {
        n_split = n_split["http://".len()..].to_string()
    }  

    
    encoder.write_all(n_split.as_bytes());
    let compressed_bytes = encoder.finish().unwrap();
    let mut encoded_base = base64::encode(compressed_bytes);


    let mut encoded_path = if splitUrl.len() > 1 {
        let fpath = splitUrl[1..].concat();
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(fpath.as_bytes()).expect("write_failed");
        let compressed_bytes: Vec<u8> = encoder.finish().unwrap();
        let encoded = base64::encode(compressed_bytes);
        encoded
    } else {
        "".to_string()
    };

    encoded_base = encoded_base.trim_matches('"').to_string();
    encoded_path = encoded_path.trim_matches('"').to_string();

    info!("encoded_path: {:?}", encoded_path);
    info!("encoded_base: {:?}", encoded_base);

    let final_tinified = if encoded_path.len() > 0 {
        format!("http://{}.co/{:?}", encoded_base, encoded_path)
    } else {
        format!("http://{}.co", encoded_base)
    };

    let resp = TinifyResponse {
        tinified_url: final_tinified
    };
    web::Json(resp)
}

#[post("/detinify")]
async fn detinify(payload: web::Json<DetinifyPayload>) -> impl Responder {
    let tinified_url = payload.tinified_url.clone();
    let sp = tinified_url.split("/").into_iter().collect::<Vec<_>>();
    let decompressed = if sp.len() > 1 {
                                let compressed_path = sp[1..].concat();
                                let mut decoder = ZlibDecoder::new(compressed_path.as_bytes());
                                let mut decompressed = String::new();
                                decoder.read_to_string(&mut decompressed).expect("decompression failed");
                                decompressed
                            } else {
                                "".to_string()
                            };

    web::Json(DetinifyResponse {
        status: Status::SUCCESS,
        detinified_url: decompressed
    })

}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    Logger::try_with_str("debug")
    .unwrap()
    .log_to_file(FileSpec::default().directory("log_files").basename("actix_server").suffix("log"))
    .duplicate_to_stdout(Duplicate::Debug)
    .start()
    .unwrap();

    info!("Starting Actix server on host 0.0.0.0 on port 8000!");


    HttpServer::new(|| {
        App::new()
        .service(tget)
        .service(tinify)
        .service(detinify)
    }).bind(("0.0.0.0", 8000))?.run().await
}
