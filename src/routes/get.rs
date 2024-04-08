use actix_web::{http::header::ContentType, web, HttpResponse, Responder};

pub async fn server_list() -> impl Responder {
  let servers = crate::servers::get_list();
  let res_body = serde_json::to_string(&servers).unwrap();
  HttpResponse::Ok()
    .content_type(ContentType::json())
    .body(res_body)
}

pub async fn server(path: web::Path<(String, u16)>) -> impl Responder {
  let (address, port) = path.into_inner();
  let servers = crate::servers::get_list();
  if let Some(i) = servers
    .iter()
    .find(|i| i.address == address && i.port == port)
  {
    let res_body = serde_json::to_string(i).unwrap();
    return HttpResponse::Ok()
      .content_type(ContentType::json())
      .body(res_body);
  }
  HttpResponse::NotFound().finish()
}

pub async fn count() -> impl Responder {
  let servers = crate::servers::get_list();
  let res_body = json!({
    "server_count": servers.len(),
    "player_count": _get_total_player_count(&servers)
  })
  .to_string();
  HttpResponse::Ok()
    .content_type(ContentType::json())
    .body(res_body)
}

pub async fn all() -> impl Responder {
  let servers = crate::servers::get_list();
  let res_body = json!({
    "server_count": servers.len(),
    "player_count": _get_total_player_count(&servers),
    "servers": servers
  })
  .to_string();
  HttpResponse::Ok()
    .content_type(ContentType::json())
    .body(res_body)
}

fn _get_total_player_count(servers: &[crate::servers::structs::Server]) -> usize {
  let mut result: usize = 0;
  servers.iter().for_each(|i| result += i.players as usize);
  result
}
