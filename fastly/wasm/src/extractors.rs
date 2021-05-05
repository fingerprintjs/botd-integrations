use fastly::http::HeaderValue;

pub fn extract_header_value(h: Option<&HeaderValue>) -> Option<String> {
    if h.is_none() {
        return Option::None;
    }
    return Option::Some(h.unwrap().to_str().unwrap_or_default().to_string());
}

pub fn extract_cookie_element(cookie: &str, element_name: &str) -> Option<String> {
    let position_option = cookie.find(element_name);
    if position_option.is_none() {
        return Option::None;
    }

    let mut cookie_value: String = String::new();
    let position = position_option.unwrap() + element_name.len();
    for i in position..cookie.len() {
        let c = cookie.chars().nth(i).unwrap();
        if c == ' ' || c == ';' {
            break
        }
        cookie_value.push(c);
    }
    return Option::Some(cookie_value);
}