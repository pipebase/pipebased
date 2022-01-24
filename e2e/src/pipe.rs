// #[cfg(feature = "itest")]
#[cfg(test)]
mod tests {
    use crate::utils::{build_client, wait};
    use pipebased_common::grpc::daemon::{
        AppDescriptor, CatalogsDescriptor, CreatePipeRequest, EnvironmentVariable, ListPipeRequest,
        PullAppRequest, PullCatalogsRequest, RemovePipeRequest, StartPipeRequest, StopPipeRequest,
    };

    const TEST_CLI_CONFIG_FILE_PATH: &str = "resources/cli.yml";
    const TEST_NAMESPACE: &str = "dev";
    const TEST_PROJECT_ID: &str = "timer";
    const TEST_BUILD_VERSION: u64 = 0;
    const TEST_CATALOGS_VERSION: u64 = 0;
    const TEST_PIPE_ID: &str = "pipebase.dev.timer";
    const TEST_PIPE_DESCRIPTION: &str = "pipebase timer app";
    const TEST_USER: &str = "pipebase";
    const TEST_GROUP: &str = "pipebase";
    const TEST_ENV_FORMATTER_KEY: &str = "PIPEBASE_LOG_FORMATTER";
    const TEST_ENV_FORMATTER_VALUE: &str = "json";
    const TEST_ENV_RUST_LOG_KEY: &str = "RUST_LOG";
    const TEST_ENV_RUST_LOG_VALUE: &str = "info";

    #[tokio::test]
    async fn test_pipe_status() {
        let mut client = build_client(TEST_CLI_CONFIG_FILE_PATH)
            .await
            .expect("build client failed");
        let resp = client
            .list_pipe(ListPipeRequest {})
            .await
            .expect("list pipe failed")
            .into_inner();
        let pipes = resp.pipes;
        assert_eq!(1, pipes.len());
        let pipe = pipes.get(0).expect("pipe state not found");
        println!("{:#?}", pipe);
    }

    #[tokio::test]
    async fn test_pipe() {
        let mut client = build_client(TEST_CLI_CONFIG_FILE_PATH)
            .await
            .expect("build client failed");
        // pull app
        client
            .pull_app(PullAppRequest {
                namespace: String::from(TEST_NAMESPACE),
                id: String::from(TEST_PROJECT_ID),
                version: TEST_BUILD_VERSION,
            })
            .await
            .expect("pull app failed");
        // pull catalogs
        client
            .pull_catalogs(PullCatalogsRequest {
                namespace: String::from(TEST_NAMESPACE),
                id: String::from(TEST_PROJECT_ID),
                version: TEST_CATALOGS_VERSION,
            })
            .await
            .expect("pull catalogs failed");
        // create pipe
        client
            .create_pipe(build_create_test_pipe_request())
            .await
            .expect("create pipe failed");
        // wait for configuration loaded
        wait(3000).await;
        // check pipe status
        let resp = client
            .list_pipe(ListPipeRequest {})
            .await
            .expect("list pipe failed")
            .into_inner();
        let pipes = resp.pipes;
        assert_eq!(1, pipes.len());
        let pipe = pipes.get(0).expect("pipe state not found");
        println!("{:#?}", pipe);
        // start pipe
        client
            .start_pipe(StartPipeRequest {
                id: String::from(TEST_PIPE_ID),
            })
            .await
            .expect("start pipe failed");
        // wait for service status change
        wait(3000).await;
        // check pipe status
        let resp = client
            .list_pipe(ListPipeRequest {})
            .await
            .expect("list pipe failed")
            .into_inner();
        let pipes = resp.pipes;
        assert_eq!(1, pipes.len());
        let pipe = pipes.get(0).expect("pipe state not found");
        println!("{:#?}", pipe);
        // wait for 3 seconds, pipe is still running
        wait(5000).await;
        let resp = client
            .list_pipe(ListPipeRequest {})
            .await
            .expect("list pipe failed")
            .into_inner();
        let pipes = resp.pipes;
        assert_eq!(1, pipes.len());
        let pipe = pipes.get(0).expect("pipe state not found");
        println!("{:#?}", pipe);
        // stop pipe
        client
            .stop_pipe(StopPipeRequest {
                id: String::from(TEST_PIPE_ID),
            })
            .await
            .expect("stop pipe failed");
        // check pipe status
        let resp = client
            .list_pipe(ListPipeRequest {})
            .await
            .expect("list pipe failed")
            .into_inner();
        let pipes = resp.pipes;
        assert_eq!(1, pipes.len());
        let pipe = pipes.get(0).expect("pipe state not found");
        println!("{:#?}", pipe);
        // remove pipe
    }

    fn build_app_descriptor(namespace: &str, id: &str, version: u64) -> AppDescriptor {
        AppDescriptor {
            namespace: String::from(namespace),
            id: String::from(id),
            version,
        }
    }

    fn build_catalogs_descriptor(namespace: &str, id: &str, version: u64) -> CatalogsDescriptor {
        CatalogsDescriptor {
            namespace: String::from(namespace),
            id: String::from(id),
            version,
        }
    }

    fn build_envs(envs: &[(&str, &str)]) -> Vec<EnvironmentVariable> {
        envs.iter()
            .map(|&(key, value)| EnvironmentVariable {
                key: String::from(key),
                value: String::from(value),
            })
            .collect()
    }

    fn build_create_test_pipe_request() -> CreatePipeRequest {
        CreatePipeRequest {
            id: String::from(TEST_PIPE_ID),
            description: Some(String::from(TEST_PIPE_DESCRIPTION)),
            user: Some(String::from(TEST_USER)),
            group: Some(String::from(TEST_GROUP)),
            envs: build_envs(&[
                (TEST_ENV_FORMATTER_KEY, TEST_ENV_FORMATTER_VALUE),
                (TEST_ENV_RUST_LOG_KEY, TEST_ENV_RUST_LOG_VALUE),
            ]),
            app: Some(build_app_descriptor(
                TEST_NAMESPACE,
                TEST_PROJECT_ID,
                TEST_BUILD_VERSION,
            )),
            catalogs: Some(build_catalogs_descriptor(
                TEST_NAMESPACE,
                TEST_PROJECT_ID,
                TEST_CATALOGS_VERSION,
            )),
        }
    }
}
