use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    response::Response,
};
use http_body_util::BodyExt;
use sdd_workshop_weather_api::{app, db, frontend};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;

async fn get(uri: &str) -> Response {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("in-memory SQLite database should open");
    db::migrate(&pool).await.expect("migrations should succeed");
    app(pool)
        .oneshot(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should handle the request")
}

async fn body_text(response: Response) -> String {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn header_str(response: &Response, name: header::HeaderName) -> &str {
    response.headers().get(name).unwrap().to_str().unwrap()
}

#[tokio::test]
async fn index_is_html_with_security_headers() {
    let response = get("/").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        header_str(&response, header::CONTENT_TYPE),
        "text/html; charset=utf-8"
    );
    assert_eq!(
        header_str(&response, header::CONTENT_SECURITY_POLICY),
        frontend::CONTENT_SECURITY_POLICY
    );
    assert_eq!(
        header_str(&response, header::X_CONTENT_TYPE_OPTIONS),
        "nosniff"
    );
    assert_eq!(
        header_str(&response, header::REFERRER_POLICY),
        "no-referrer"
    );
    assert!(body_text(response).await.contains("Weather Globe"));
}

#[tokio::test]
async fn index_has_no_inline_script_and_pins_globe_library() {
    let html = frontend::INDEX.body;
    for tag in html.split("<script").skip(1) {
        let tag = &tag[..tag.find('>').unwrap()];
        assert!(tag.contains("src="), "inline script found: <script{tag}>");
    }
    assert!(html.contains("globe.gl@2.46.2"));
    assert!(html.contains("integrity=\"sha384-"));
    assert!(html.contains("crossorigin=\"anonymous\""));
    assert!(!html.contains(" onclick=") && !html.contains(" onload="));
}

#[tokio::test]
async fn static_assets_have_correct_types_and_headers() {
    for (path, content_type) in [
        ("/static/js/main.js", "text/javascript; charset=utf-8"),
        ("/static/css/app.css", "text/css; charset=utf-8"),
        ("/static/favicon.svg", "image/svg+xml"),
    ] {
        let response = get(path).await;
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert_eq!(header_str(&response, header::CONTENT_TYPE), content_type);
        assert!(
            response
                .headers()
                .contains_key(header::CONTENT_SECURITY_POLICY)
        );
    }
}

#[tokio::test]
async fn unknown_and_traversal_paths_are_not_found() {
    for path in [
        "/static/",
        "/static",
        "/static/missing.js",
        "/static/../Cargo.toml",
        "/static/%2e%2e/Cargo.toml",
        "/static/js/../../Cargo.toml",
        "/static//etc/passwd",
        "/static/index.html",
    ] {
        let response = get(path).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
    }
}

#[tokio::test]
async fn api_routes_do_not_get_the_frontend_csp() {
    for path in [
        "/api/weather",
        "/api/weather/forecast/London",
        "/v3/api-docs",
    ] {
        let response = get(path).await;
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert!(
            !response
                .headers()
                .contains_key(header::CONTENT_SECURITY_POLICY),
            "{path}"
        );
    }
}

#[test]
fn embedded_files_avoid_unsafe_dom_and_eval_apis() {
    const FORBIDDEN: &[&str] = &[
        "innerHTML",
        "outerHTML",
        "insertAdjacentHTML",
        "document.write",
        "eval(",
        "new Function",
        "setAttribute('style'",
        "setAttribute(\"style\"",
        "setAttribute('on",
        "setAttribute(\"on",
        "setTimeout('",
        "setTimeout(\"",
        "setInterval('",
        "setInterval(\"",
    ];
    let mut files: Vec<(&str, &str)> = frontend::ASSETS
        .iter()
        .filter(|a| a.path.ends_with(".js"))
        .map(|a| (a.path, a.body))
        .collect();
    files.push((frontend::INDEX.path, frontend::INDEX.body));
    for (path, body) in files {
        for token in FORBIDDEN {
            assert!(
                !body.contains(token),
                "{path} contains forbidden token {token}"
            );
        }
    }
    // only api.js may call fetch
    for asset in frontend::ASSETS.iter().filter(|a| a.path.ends_with(".js")) {
        if asset.path != "js/api.js" {
            assert!(!asset.body.contains("fetch("), "{} calls fetch", asset.path);
        }
    }
}

#[tokio::test]
async fn favourites_module_is_served_with_csp_and_wired_into_the_page() {
    let response = get("/static/js/favourites.js").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        header_str(&response, header::CONTENT_TYPE),
        "text/javascript; charset=utf-8"
    );
    assert_eq!(
        header_str(&response, header::CONTENT_SECURITY_POLICY),
        frontend::CONTENT_SECURITY_POLICY
    );

    let html = frontend::INDEX.body;
    for id in [
        "id=\"favourites\"",
        "id=\"favourite-list\"",
        "id=\"fav-toggle\"",
    ] {
        assert!(html.contains(id), "index.html is missing {id}");
    }
    let asset = |path: &str| {
        frontend::ASSETS
            .iter()
            .find(|a| a.path == path)
            .unwrap_or_else(|| panic!("{path} is not registered"))
            .body
    };
    assert!(asset("css/app.css").contains(".pin.favourite"));
    let api = asset("js/api.js");
    for call in ["/api/favourites", "'POST'", "'DELETE'"] {
        assert!(api.contains(call), "api.js is missing {call}");
    }
    assert!(asset("js/ui.js").contains("fav-item"));
}

/// DOM `append()` stringifies `null`/`undefined`/`false`, so a conditional argument such as
/// `cond ? el(..) : null` renders the literal text "null". Returns the offending arguments.
fn append_args_with_nullish_branches(source: &str) -> Vec<String> {
    let mut bad = Vec::new();
    let mut from = 0;
    while let Some(found) = source[from..].find(".append(") {
        let start = from + found + ".append(".len();
        let (mut depth, mut quote) = (0usize, None::<char>);
        let (mut arg, mut end) = (String::new(), start);
        for (offset, c) in source[start..].char_indices() {
            end = start + offset;
            if let Some(q) = quote {
                if c == q {
                    quote = None;
                }
                arg.push(' ');
                continue;
            }
            match c {
                '\'' | '"' | '`' => quote = Some(c),
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' if depth == 0 => break,
                ')' | ']' | '}' => depth -= 1,
                ',' if depth == 0 => {
                    bad.extend(nullish_branch(&arg));
                    arg.clear();
                    continue;
                }
                _ => {}
            }
            if depth == 0 || !matches!(c, '(' | '[' | '{') {
                if depth == 0 {
                    arg.push(c);
                } else {
                    arg.push(' ');
                }
            }
        }
        bad.extend(nullish_branch(&arg));
        from = end + 1;
    }
    bad
}

fn nullish_branch(arg: &str) -> Option<String> {
    let arg = arg.trim();
    let nullish = [": null", ": undefined", ": false", "&& ", "|| null"];
    (arg.contains('?') && nullish.iter().any(|n| arg.contains(n))
        || arg.contains("&&")
        || arg.contains("|| null"))
    .then(|| arg.to_owned())
}

#[test]
fn append_scanner_detects_nullish_arguments() {
    assert!(
        !append_args_with_nullish_branches("node.append(a, fav ? el('b') : null, c)").is_empty()
    );
    assert!(!append_args_with_nullish_branches("node.append(fav && el('b'))").is_empty());
    assert!(
        append_args_with_nullish_branches("node.append(el('li', null, x ? null : y), c)")
            .is_empty()
    );
    assert!(append_args_with_nullish_branches("if (f) node.append(el('b'))").is_empty());
}

#[test]
fn append_calls_never_receive_nullish_conditional_arguments() {
    for asset in frontend::ASSETS.iter().filter(|a| a.path.ends_with(".js")) {
        let bad = append_args_with_nullish_branches(asset.body);
        assert!(
            bad.is_empty(),
            "{} passes a possibly-null argument to append(): {bad:?}",
            asset.path
        );
    }
}
