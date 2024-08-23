use super::base::PlatzClient;
use super::error::PlatzClientError;
use async_trait::async_trait;
use reqwest::{ClientBuilder, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use tracing::instrument;

#[derive(Clone)]
pub struct PlatzRequest<'a> {
    client: &'a PlatzClient,
    method: reqwest::Method,
    path: String,
    query: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct Paginated<T> {
    pub page: i64,
    pub per_page: i64,
    pub items: Vec<T>,
    pub num_total: i64,
}

lazy_static::lazy_static! {
    static ref HTTP_USER_AGENT: String = format!(
        "{}/{}/{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        option_env!("CARGO_BIN_NAME").unwrap_or("lib")
    );
}

impl<'a> PlatzRequest<'a> {
    pub fn new<S>(client: &'a PlatzClient, method: reqwest::Method, path: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            client,
            method,
            path: path.as_ref().to_owned(),
            query: Default::default(),
        }
    }

    pub fn query<K, V>(mut self, key: K, value: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.query
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
        self
    }

    pub fn add_to_query<I, K, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (key, value) in iter.into_iter() {
            self.query
                .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
        }
        self
    }

    pub async fn request_builder(&self) -> Result<RequestBuilder, PlatzClientError> {
        let (header_key, header_value) = self.client.authorization().await?;
        Ok(ClientBuilder::new()
            .user_agent(HTTP_USER_AGENT.clone())
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .build()?
            .request(
                self.method.clone(),
                self.client.build_url(&self.path).await?,
            )
            .header(header_key, header_value)
            .query(&self.query))
    }

    pub async fn send_with_no_response(self) -> Result<(), PlatzClientError> {
        self.request_builder()
            .await?
            .send()
            .await?
            .error_for_status_with_body()
            .await?;
        Ok(())
    }

    pub async fn send<T>(self) -> Result<T, PlatzClientError>
    where
        T: DeserializeOwned + Send,
    {
        Ok(self
            .request_builder()
            .await?
            .send()
            .await?
            .error_for_status_with_body()
            .await?
            .json()
            .await?)
    }

    #[instrument(skip_all, fields(path=self.path))]
    pub async fn send_with_body<T, R>(self, body: T) -> Result<R, PlatzClientError>
    where
        T: Serialize,
        R: DeserializeOwned + Send,
    {
        Ok(self
            .request_builder()
            .await?
            .json(&body)
            .send()
            .await?
            .error_for_status_with_body()
            .await?
            .json()
            .await?)
    }

    pub async fn single_page<T>(
        &self,
        page_index: i64,
        page_size: Option<i64>,
    ) -> Result<Paginated<T>, PlatzClientError>
    where
        T: DeserializeOwned + Send,
        Paginated<T>: DeserializeOwned + Send,
    {
        let mut paging_info = vec![("page", page_index.to_string())];
        if let Some(size) = page_size {
            paging_info.push(("page_size", size.to_string()))
        }
        let page = self
            .request_builder()
            .await?
            .query(&paging_info)
            .send()
            .await?
            .error_for_status_with_body()
            .await?
            .json()
            .await?;
        Ok(page)
    }

    #[instrument(skip_all, fields(path=self.path))]
    pub async fn paginated<T>(self) -> Result<Vec<T>, PlatzClientError>
    where
        T: DeserializeOwned + Send,
        Paginated<T>: DeserializeOwned + Send,
    {
        let page_size: Option<i64> = None;
        let mut cur_page = self.single_page(1, page_size).await?;
        let mut items = cur_page.items;

        while cur_page.page * cur_page.per_page < cur_page.num_total {
            let next_page = cur_page.page + 1;
            cur_page = self.single_page(next_page, page_size).await?;
            items.extend(cur_page.items.into_iter());
        }

        Ok(items)
    }

    #[instrument(skip_all, fields(path=self.path))]
    pub async fn paginated_expect_one<T>(self) -> Result<T, PlatzClientError>
    where
        T: DeserializeOwned + Send,
        Paginated<T>: DeserializeOwned + Send,
    {
        let items = self.paginated().await?;
        match items.len() {
            0 => Err(PlatzClientError::ExpectedOneGotNone),
            1 => Ok(items.into_iter().next().unwrap()),
            n => Err(PlatzClientError::ExpectedOneGotMany(n)),
        }
    }
}
#[async_trait]
trait ResponseExt {
    async fn error_for_status_with_body(self) -> Result<reqwest::Response, PlatzClientError>;
}

#[async_trait]
impl ResponseExt for reqwest::Response {
    async fn error_for_status_with_body(self) -> Result<reqwest::Response, PlatzClientError> {
        let status = self.status();
        if status.is_success() {
            Ok(self)
        } else {
            let body = self
                .text()
                .await
                .ok()
                .map(|s| format!(": {s:?}"))
                .unwrap_or_default();
            let err_msg = format!("{status}{body}");
            Err(PlatzClientError::HttpError(err_msg))
        }
    }
}
