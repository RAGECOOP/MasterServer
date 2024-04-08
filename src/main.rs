#[macro_use]
extern crate serde_json;

use std::sync::OnceLock;

use actix_web::{dev::Service as _, web, App, HttpServer};

#[macro_use]
mod logger;
mod config;
mod routes;
mod servers;

pub fn get_current_dir() -> &'static String {
  static MEM: OnceLock<String> = OnceLock::new();
  MEM.get_or_init(|| {
    let dir = if let Ok(r) = std::env::current_dir() {
      r.display().to_string()
    } else {
      // Shouldn't happen
      String::from(".\\")
    };
    dir
  })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let conf = config::load_config();

  log!("info", format!("port: {}", conf.server.port));
  log!("info", format!("workers: {}", conf.server.workers));
  log!(
    "starting",
    format!(
      "server on http://{}:{}",
      conf.server.address,
      conf.server.port
    )
  );

  HttpServer::new(|| {
    let cors = actix_cors::Cors::default()
      .allow_any_origin()
      .allowed_methods(vec!["GET", "POST"]);

    App::new()
      .wrap(cors)
      .wrap_fn(|req, srv| {
        servers::cleanup();
        let fut = srv.call(req);
        async {
          let res = fut.await?;
          Ok(res)
        }
      })
      .route("/", web::post().to(routes::post::server))
      .route("/", web::get().to(routes::get::server_list))
      .route("/{address}:{port}", web::get().to(routes::get::server))
      .route("/all", web::get().to(routes::get::all))
      .route("/count", web::get().to(routes::get::count))
  })
  .workers(conf.server.workers as usize)
  .bind((conf.server.address, conf.server.port))?
  .run()
  .await
}
