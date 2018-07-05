use std::sync::Arc;

use hyper::{self, Server};

use super::super::{context::Context, env, errors::Result, router};

pub fn server(cfg: &env::Http, ctx: Arc<Context>) -> Result<()> {
    // let addr = ([127, 0, 0, 1], cfg.port).into();

    // let proxy = router::RouterBuilder::new(ctx, Arc::new(router::routes()?));
    // let srv = Server::bind(&addr)
    //     .serve(proxy)
    //     .map_err(|e| eprintln!("server error: {:?}", e));
    //
    // info!("Listening on http://{}", addr);
    // hyper::rt::run(srv);
    Ok(())
}

pub fn routes() -> Result<()> {
    println!("{}\t{}", "METHOD", "PATH");
    // for (m, p, _) in router::routes()? {
    //     println!("{}\t{}", m, p);
    // }
    Ok(())
}
