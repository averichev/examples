//! Example of cookie based session
//! Session data is stored in cookie, it is limited to 4kb
//!
//! [Redis session example](https://github.com/actix/examples/tree/master/redis-session)
//!
//! [User guide](https://actix.rs/docs/middleware/#user-sessions)

use actix_session::{CookieSession, Session};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Result};

/// simple index handler with session
async fn index(session: Session, req: HttpRequest) -> Result<&'static str> {
    log::info!("{:?}", req);

    // RequestSession trait is used for session access
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        log::info!("SESSION value: {}", count);
        counter = count + 1;
        session.insert("counter", counter)?;
    } else {
        session.insert("counter", counter)?;
    }

    Ok("welcome!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    log::info!("Starting http server: 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(Logger::default())
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
