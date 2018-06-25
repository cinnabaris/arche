error_chain!{
    foreign_links {
        StdIo(std::io::Error);
        SerdeJson(serde_json::Error);
        Redis(redis::RedisError);
        R2d2(r2d2::Error);
        ChronoParse(chrono::ParseError);
        Diesel(diesel::result::Error);
    }
}
