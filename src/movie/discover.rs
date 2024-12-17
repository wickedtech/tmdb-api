use std::borrow::Cow;

const PATH: &str = "/discover/movie";

#[derive(Clone, Debug, Default)]
pub struct MovieDiscover {
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
    /// Which page to query.
    pub page: Option<u32>,
    /// Whether to include adult (pornography) content in the results.
    pub include_adult: bool,
    /// ISO 3166-1 code to filter release region. Must be uppercase.
    pub region: Option<String>,
    /// ISO 3166-1 code to filter release region. Must be uppercase.
    pub watch_region: Option<String>,
    /// ISO 3166-1 code to filter release region. Must be uppercase.
    pub with_origin_country: Option<String>,
    /// Language code. Must be lowercase.
    pub with_original_language: Option<String>,
    /// Sort By
    pub sort_by: Option<String>,
    /// With watch monetization types
    pub with_watch_monetization_types: Option<String>,
}

impl MovieDiscover {
    pub fn new() -> Self {
        Self {
            language: None,
            page: None,
            include_adult: false,
            region: None,
            watch_region: None,
            with_origin_country: None,
            with_original_language: None,
            sort_by: Some("popularity.desc".into()),
            with_watch_monetization_types: None,
        }
    }

    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }

    pub fn with_page(mut self, value: Option<u32>) -> Self {
        self.page = value;
        self
    }

    pub fn with_include_adult(mut self, value: bool) -> Self {
        self.include_adult = value;
        self
    }

    pub fn with_region(mut self, value: Option<String>) -> Self {
        self.region = value;
        self
    }

    pub fn with_watch_region(mut self, value: Option<String>) -> Self {
        self.watch_region = value;
        self
    }

    pub fn with_origin_country(mut self, value: Option<String>) -> Self {
        self.with_origin_country = value;
        self
    }

    pub fn with_original_language(mut self, value: Option<String>) -> Self {
        self.with_original_language = value;
        self
    }

    pub fn with_sort_by(mut self, value: Option<String>) -> Self {
        self.sort_by = value;
        self
    }

    pub fn with_watch_monetization_types(mut self, value: Option<String>) -> Self {
        self.with_watch_monetization_types = value;
        self
    }
}

impl crate::prelude::Command for MovieDiscover {
    type Output = crate::common::PaginatedResult<super::MovieShort>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(PATH)
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = vec![];

        if let Some(language) = self.language.as_ref() {
            res.push(("language", Cow::Borrowed(language.as_str())));
        }
        if let Some(page) = self.page {
            res.push(("page", Cow::Owned(page.to_string())));
        }
        if self.include_adult {
            res.push(("include_adult", Cow::Borrowed("true")));
        }
        if let Some(region) = self.region.as_ref() {
            res.push(("region", Cow::Borrowed(region.as_str())));
        }
        if let Some(watch_region) = self.watch_region.as_ref() {
            res.push(("watch_region", Cow::Owned(watch_region.to_string())));
        }
        if let Some(with_origin_country) = self.with_origin_country.as_ref() {
            res.push((
                "with_origin_country",
                Cow::Owned(with_origin_country.to_string()),
            ));
        }
        if let Some(with_original_language) = self.with_original_language.as_ref() {
            res.push((
                "with_original_language",
                Cow::Owned(with_original_language.to_string()),
            ));
        }
        if let Some(sort_by) = self.sort_by.as_ref() {
            res.push(("sort_by", Cow::Borrowed(sort_by.as_str())));
        }
        if let Some(with_watch_monetization_types) = self.with_watch_monetization_types.as_ref() {
            res.push((
                "with_watch_monetization_types",
                Cow::Owned(with_watch_monetization_types.to_string()),
            ));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::MovieDiscover;
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

        let cmd = MovieDiscover::new();

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "api_key".into(),
                "secret".into(),
            )]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/discover-movie.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert!(!result.results.is_empty());
        assert!(result.total_pages > 0);
        assert!(result.total_results > 0);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.title, "Le clitoris");
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let cmd = MovieDiscover::new();

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "api_key".into(),
                "secret".into(),
            )]))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
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

        let cmd = MovieDiscover::new();

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "api_key".into(),
                "secret".into(),
            )]))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }

    #[tokio::test]
    async fn validation_error() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let cmd = MovieDiscover::new();

        let _m = server
            .mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "api_key".into(),
                "secret".into(),
            )]))
            .with_status(422)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/validation-error.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
        let validation_err = err.as_validation_error().unwrap();
        assert_eq!(validation_err.errors.len(), 1);
    }

    // #[tokio::test]
    // async fn premature_end_of_line() {
    // let mut server = mockito::Server::new_async().await;
    // let client = Client::<ReqwestExecutor>::builder().with_api_key("secret".into()).with_base_url(server.url()).build().unwrap();

    //     let client = Client::<ReqwestExecutor>::new("secret".into()).with_base_url(mockito::server_url());
    //     let cmd = MovieSearch::new("game of thrones".into());

    //     let _m = mock("GET", super::PATH)
    //         .match_query(Matcher::AllOf(vec![
    //             Matcher::UrlEncoded("api_key".into(), "secret".into()),
    //             Matcher::UrlEncoded("query".into(), "game of thrones".into()),
    //         ]))
    //         .with_status(200)
    //         .with_header("content-type", "application/json;charset=utf-8")
    //         .with_body(include_str!("../../assets/search-tv-decoding-error.json"))
    //         .create_async().await;
    //     let result = cmd.execute(&client).await.unwrap();
    //     assert_eq!(result.page, 1);
    // }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::MovieSearch;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::client::Client;
    use crate::prelude::Command;

    #[tokio::test]
    async fn search_rrrrrrr() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);
        let cmd = MovieSearch::new("Rrrrrrr".into());

        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.total_pages, 1);
        assert_eq!(result.total_results, 1);
        let item = result.results.first().unwrap();
        assert_eq!(item.inner.title, "RRRrrrr!!!");
    }

    #[tokio::test]
    async fn search_simpsons() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);
        let cmd = MovieSearch::new("simpsons".into());

        let _result = cmd.execute(&client).await.unwrap();
    }
}
