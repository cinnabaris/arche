use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use nut::{context::Context, errors::Result};
use rocket::{
    self,
    config::{Config, Environment, LoggingLevel},
};
use rocket_contrib::Template;

use super::super::{graphql, router, worker};

pub fn server() -> Result<()> {
    let ctx = Context::new(&super::parse_config()?)?;
    // http
    let mut app = rocket::custom(
        Config::build(ctx.config.env()?)
            .address("localhost")
            .workers(ctx.config.workers)
            .secret_key(ctx.config.secret_key.clone())
            .port(ctx.config.http.port)
            .limits(ctx.config.http.limits())
            .extra(
                "template_dir",
                match Path::new("themes")
                    .join(&ctx.config.http.theme)
                    .join("views")
                    .to_str()
                {
                    Some(v) => v,
                    None => "views",
                },
            )
            .finalize()?,
        false,
    ).manage(ctx.db.clone())
        .manage(ctx.cache.clone())
        .manage(ctx.encryptor.clone())
        .manage(ctx.queue.clone())
        .manage(graphql::schema::Schema::new(
            graphql::query::Query {},
            graphql::mutation::Mutation {},
        ))
        .manage(ctx.config.clone());

    for (pt, rt) in router::routes() {
        app = app.mount(pt, rt);
    }

    let cors = ctx.config.http.cors();
    thread::spawn(|| {
        // Template::custom(|engines| {
        //     engines
        //         .handlebars
        //         .register_global_function("eq", format::eq);
        // })
        app.attach(Template::fairing())
            .attach(cors)
            .catch(nut::router::catchers())
            .launch();
    });

    // worker
    loop {
        match worker::consume(&Arc::new(ctx.clone())) {
            Ok(_) => info!("exiting worker"),
            Err(e) => error!("{:?}", e),
        };
        thread::sleep(Duration::from_secs(10));
    }
}

pub fn routes() -> Result<()> {
    let mut app = rocket::custom(
        Config::build(Environment::Production)
            .log_level(LoggingLevel::Critical)
            .finalize()?,
        false,
    );
    for (pt, rt) in router::routes() {
        app = app.mount(pt, rt);
    }
    println!("{:8} {:<8} {}", "METHOD", "RANK", "PATH");
    for it in app.routes() {
        println!("{:8} {:<8} {}", it.method, it.rank, it.uri.path());
    }
    Ok(())
}
