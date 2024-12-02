use std::borrow::Cow;
use crate::common::video::Video;

/// Get a list of recommended tvs for a tv shows.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::client::Client;
/// use tmdb_api::client::reqwest::ReqwestExecutor;
/// use tmdb_api::tvshow::videos::TVShowVideos;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
///     let cmd = TVShowVideos::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct TVShowVideos {
    /// ID of the tv.
    pub series_id: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

impl TVShowVideos {
    pub fn new(series_id: u64) -> Self {
        Self {
            series_id,
            language: None,
        }
    }

    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct TVShowVideosResult {
    pub id: u64,
    pub results: Vec<Video>,
}

impl crate::prelude::Command for TVShowVideos {
    type Output = TVShowVideosResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/{}/videos", self.series_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = Vec::with_capacity(1);
        if let Some(language) = self.language.as_ref() {
            res.push(("language", Cow::Borrowed(language.as_str())));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::TVShowVideos;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::client::Client;
    use crate::prelude::Command;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/550/videos")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-videos.json"))
            .create_async()
            .await;

        let result = TVShowVideos::new(550).execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
        assert!(!result.results.is_empty());
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/550/videos")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = TVShowVideos::new(550).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/550/videos")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = TVShowVideos::new(550).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::TVShowVideos;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::client::Client;
    use crate::prelude::Command;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = TVShowVideos::new(550).execute(&client).await.unwrap();
        assert_eq!(result.id, 550);
    }
}
