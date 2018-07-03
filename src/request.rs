use http::{
    header::{AsHeaderName, AUTHORIZATION, HOST, ORIGIN}, request::Parts, HeaderMap,
};

pub fn locale(parts: &Parts) -> Option<String> {
    if let Some(v) = get(&parts.headers, HOST) {
        return Some(v);
    }
    None
}
pub fn host(parts: &Parts) -> Option<String> {
    if let Some(v) = get(&parts.headers, HOST) {
        return Some(v);
    }
    None
}

pub fn origin(parts: &Parts) -> Option<String> {
    if let Some(v) = get(&parts.headers, ORIGIN) {
        return Some(v);
    }
    None
}

// https://jwt.io/introduction/
const BEARER: &'static str = "Bearer ";
pub fn token(parts: &Parts) -> Option<String> {
    if let Some(v) = get(&parts.headers, AUTHORIZATION) {
        if v.starts_with(BEARER) {
            return Some((&v[BEARER.len()..]).to_string());
        }
    }
    None
}

pub fn client_ip(parts: &Parts) -> Option<String> {
    if let Some(ip) = get(&parts.headers, "X-Forwarded-For") {
        let ip: Vec<&str> = ip.split(',').collect();
        if let Some(ip) = ip.first() {
            return Some(ip.trim().to_string());
        }
    }
    if let Some(ip) = get(&parts.headers, "X-Real-IP") {
        return Some(ip.to_string());
    }
    if let Some(ip) = get(&parts.headers, "X-Appengine-Remote-Addr") {
        return Some(ip.to_string());
    }
    None
}

fn get<K: AsHeaderName>(m: &HeaderMap, k: K) -> Option<String> {
    if let Some(v) = m.get(k) {
        if let Ok(v) = v.to_str() {
            return Some(v.to_string());
        }
    }
    None
}
