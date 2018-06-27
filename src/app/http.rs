use std::sync::Arc;

use hyper::{self, rt::Future, Server};

use super::super::{context::Context, env, errors::Result, router};

pub fn server(cfg: &env::Http, ctx: Arc<Context>) {
    let addr = ([127, 0, 0, 1], cfg.port).into();

    let proxy = router::RouterBuilder::new(ctx);
    let srv = Server::bind(&addr)
        .serve(proxy)
        .map_err(|e| eprintln!("server error: {:?}", e));

    info!("Listening on http://{}", addr);
    hyper::rt::run(srv);
}

pub fn routes() -> Result<()> {
    println!("{:8} {}", "METHOD", "PATH");
    for (m, p, _) in router::routes() {
        println!("{:8} {:<8}", m, p);
    }
    Ok(())
}
