use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use domain::repository::ItemRepository;
use infrastructure::in_memory::repository::ItemRepository as ItemRepositoryImpl;

#[get("/")]
async fn hello() -> impl Responder {
    let item_repository = di();
    let item = item_repository.find(&1);
    println!("Item id is {}", item.id);
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// TODO: ここがDI。依存関係はどこかでまとめてやる
fn di() -> impl ItemRepository {
    ItemRepositoryImpl::new()
}
