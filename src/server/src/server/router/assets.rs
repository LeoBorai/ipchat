use axum::response::{IntoResponse, Response};
use http::{Uri, header};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../ui/dist"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                (
                    [
                        (header::CONTENT_TYPE, mime.as_ref()),
                        (header::CACHE_CONTROL, "max-age=604800"),
                    ],
                    content.data,
                )
                    .into_response()
            }
            None => {
                let bytes = Asset::get("index.html")
                    .expect("index.html should exist")
                    .data;

                ([(header::CONTENT_TYPE, "text/html")], bytes).into_response()
            }
        }
    }
}

pub async fn serve_asset(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    StaticFile(path)
}
