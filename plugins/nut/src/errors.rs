error_chain!{
    foreign_links {
        SerdeJson(::serde_json::Error);
        Redis(redis::RedisError);
        R2d2(r2d2::Error);
    }
}
