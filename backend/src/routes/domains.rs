async fn add_domain_route(
  db_pool: web::Data<MySqlPool>,
  redis: web::Data<RedisPool>,
  request: HttpRequest,
  domain: web::Json<NewDomain>,
) -> HttpResponse {

}