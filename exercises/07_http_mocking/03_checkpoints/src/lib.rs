use reqwest::{Client, Url};

pub struct Repository {
    base_uri: Url,
    client: Client,
    caller_id: usize,
}

impl Repository {
    pub async fn new(base_uri: Url, caller_id: usize) -> Self {
        let self_ = Repository {
            caller_id,
            base_uri,
            client: Client::new(),
        };
        if !self_.is_authorized().await {
            panic!("Caller does not have permissions to use a repository")
        }
        self_
    }

    pub async fn get(&self, _entity_id: usize) -> Result<(), String> {
        // Permission might get revoked between the time we create the repository and the time we
        // call get. We want to check again.
        if !self.is_authorized().await {
            return Err("Caller no longer has permissions to use a repository".into());
        }

        // [...]
        Ok(())
    }

    async fn is_authorized(&self) -> bool {
        let url = self
            .base_uri
            .join(&format!("/auth/{}", self.caller_id))
            .unwrap();
        let response = self.client.get(url).send().await.unwrap();
        response.status().is_success()
    }
}

#[cfg(test)]
mod tests {
    use crate::Repository;
    use googletest::assert_that;
    use googletest::matchers::eq;
    use googletest::prelude::err;
    use reqwest::Url;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[googletest::test]
    #[tokio::test]
    async fn permissions_can_be_revoked() {
        // Arrange
        let server = MockServer::start().await;
        let caller_id = 1;
        let repository = setup_repository(&server, caller_id).await;

        server
            .register(
                Mock::given(method("GET"))
                    .and(path(&format!("/auth/{caller_id}")))
                    .respond_with(ResponseTemplate::new(403)),
            )
            .await;

        // Act
        let outcome = repository.get(2).await;

        // Assert
        assert_that!(
            outcome,
            err(eq("Caller no longer has permissions to use a repository"))
        );
    }

    async fn setup_repository(mock_server: &MockServer, caller_id: usize) -> Repository {
        let base_url = Url::parse(&mock_server.uri()).unwrap();
        let _guard = mock_server
            .register_as_scoped(
                Mock::given(method("GET"))
                    .and(path(&format!("/auth/{caller_id}")))
                    .respond_with(ResponseTemplate::new(200)),
            )
            .await;
        Repository::new(base_url.clone(), caller_id).await
    }
}
