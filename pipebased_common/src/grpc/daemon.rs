#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppDescriptor {
    /// project namespace
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// project id
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// project build version
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppResponse {
    #[prost(message, repeated, tag = "1")]
    pub apps: ::prost::alloc::vec::Vec<AppDescriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogsDescriptor {
    /// project namespace
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// project id
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// project build version
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsResponse {
    #[prost(message, repeated, tag = "1")]
    pub catalogs: ::prost::alloc::vec::Vec<CatalogsDescriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullAppRequest {
    /// project namespace
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// project id
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// project build version
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullAppResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullCatalogsRequest {
    /// project namespace
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// project id
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// project build version
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullCatalogsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAppRequest {
    /// project namespace
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// project id
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// project build version
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAppResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCatalogsRequest {
    /// project namespace
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// project id
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// project build version
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCatalogsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePipeRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub app: ::core::option::Option<AppDescriptor>,
    #[prost(message, optional, tag = "6")]
    pub catalogs: ::core::option::Option<CatalogsDescriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePipeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPipeRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPipeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopPipeRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopPipeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePipeRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePipeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPipeRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipeState {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub load_state: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub active_state: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub sub_state: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPipeResponse {
    #[prost(message, repeated, tag = "1")]
    pub pipes: ::prost::alloc::vec::Vec<PipeState>,
}
#[doc = r" Generated client implementations."]
pub mod daemon_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DaemonClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DaemonClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DaemonClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DaemonClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DaemonClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " repository operations"]
        pub async fn list_app(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAppRequest>,
        ) -> Result<tonic::Response<super::ListAppResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/ListApp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_catalogs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCatalogsRequest>,
        ) -> Result<tonic::Response<super::ListCatalogsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/ListCatalogs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn pull_app(
            &mut self,
            request: impl tonic::IntoRequest<super::PullAppRequest>,
        ) -> Result<tonic::Response<super::PullAppResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/PullApp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn pull_catalogs(
            &mut self,
            request: impl tonic::IntoRequest<super::PullCatalogsRequest>,
        ) -> Result<tonic::Response<super::PullCatalogsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/PullCatalogs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_app(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveAppRequest>,
        ) -> Result<tonic::Response<super::RemoveAppResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/RemoveApp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_catalogs(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveCatalogsRequest>,
        ) -> Result<tonic::Response<super::RemoveCatalogsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/RemoveCatalogs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " pipe operations"]
        pub async fn create_pipe(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePipeRequest>,
        ) -> Result<tonic::Response<super::CreatePipeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/CreatePipe");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn start_pipe(
            &mut self,
            request: impl tonic::IntoRequest<super::StartPipeRequest>,
        ) -> Result<tonic::Response<super::StartPipeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/StartPipe");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stop_pipe(
            &mut self,
            request: impl tonic::IntoRequest<super::StopPipeRequest>,
        ) -> Result<tonic::Response<super::StopPipeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/StopPipe");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_pipe(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePipeRequest>,
        ) -> Result<tonic::Response<super::DeletePipeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/DeletePipe");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_pipe(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPipeRequest>,
        ) -> Result<tonic::Response<super::ListPipeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemon.Daemon/ListPipe");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod daemon_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DaemonServer."]
    #[async_trait]
    pub trait Daemon: Send + Sync + 'static {
        #[doc = " repository operations"]
        async fn list_app(
            &self,
            request: tonic::Request<super::ListAppRequest>,
        ) -> Result<tonic::Response<super::ListAppResponse>, tonic::Status>;
        async fn list_catalogs(
            &self,
            request: tonic::Request<super::ListCatalogsRequest>,
        ) -> Result<tonic::Response<super::ListCatalogsResponse>, tonic::Status>;
        async fn pull_app(
            &self,
            request: tonic::Request<super::PullAppRequest>,
        ) -> Result<tonic::Response<super::PullAppResponse>, tonic::Status>;
        async fn pull_catalogs(
            &self,
            request: tonic::Request<super::PullCatalogsRequest>,
        ) -> Result<tonic::Response<super::PullCatalogsResponse>, tonic::Status>;
        async fn remove_app(
            &self,
            request: tonic::Request<super::RemoveAppRequest>,
        ) -> Result<tonic::Response<super::RemoveAppResponse>, tonic::Status>;
        async fn remove_catalogs(
            &self,
            request: tonic::Request<super::RemoveCatalogsRequest>,
        ) -> Result<tonic::Response<super::RemoveCatalogsResponse>, tonic::Status>;
        #[doc = " pipe operations"]
        async fn create_pipe(
            &self,
            request: tonic::Request<super::CreatePipeRequest>,
        ) -> Result<tonic::Response<super::CreatePipeResponse>, tonic::Status>;
        async fn start_pipe(
            &self,
            request: tonic::Request<super::StartPipeRequest>,
        ) -> Result<tonic::Response<super::StartPipeResponse>, tonic::Status>;
        async fn stop_pipe(
            &self,
            request: tonic::Request<super::StopPipeRequest>,
        ) -> Result<tonic::Response<super::StopPipeResponse>, tonic::Status>;
        async fn delete_pipe(
            &self,
            request: tonic::Request<super::DeletePipeRequest>,
        ) -> Result<tonic::Response<super::DeletePipeResponse>, tonic::Status>;
        async fn list_pipe(
            &self,
            request: tonic::Request<super::ListPipeRequest>,
        ) -> Result<tonic::Response<super::ListPipeResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DaemonServer<T: Daemon> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Daemon> DaemonServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DaemonServer<T>
    where
        T: Daemon,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/daemon.Daemon/ListApp" => {
                    #[allow(non_camel_case_types)]
                    struct ListAppSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::ListAppRequest> for ListAppSvc<T> {
                        type Response = super::ListAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_app(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/ListCatalogs" => {
                    #[allow(non_camel_case_types)]
                    struct ListCatalogsSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::ListCatalogsRequest> for ListCatalogsSvc<T> {
                        type Response = super::ListCatalogsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCatalogsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_catalogs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListCatalogsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/PullApp" => {
                    #[allow(non_camel_case_types)]
                    struct PullAppSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::PullAppRequest> for PullAppSvc<T> {
                        type Response = super::PullAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PullAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pull_app(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PullAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/PullCatalogs" => {
                    #[allow(non_camel_case_types)]
                    struct PullCatalogsSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::PullCatalogsRequest> for PullCatalogsSvc<T> {
                        type Response = super::PullCatalogsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PullCatalogsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pull_catalogs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PullCatalogsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/RemoveApp" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveAppSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::RemoveAppRequest> for RemoveAppSvc<T> {
                        type Response = super::RemoveAppResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveAppRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_app(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveAppSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/RemoveCatalogs" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveCatalogsSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::RemoveCatalogsRequest> for RemoveCatalogsSvc<T> {
                        type Response = super::RemoveCatalogsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveCatalogsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_catalogs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveCatalogsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/CreatePipe" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePipeSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::CreatePipeRequest> for CreatePipeSvc<T> {
                        type Response = super::CreatePipeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePipeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_pipe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePipeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/StartPipe" => {
                    #[allow(non_camel_case_types)]
                    struct StartPipeSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::StartPipeRequest> for StartPipeSvc<T> {
                        type Response = super::StartPipeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartPipeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_pipe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartPipeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/StopPipe" => {
                    #[allow(non_camel_case_types)]
                    struct StopPipeSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::StopPipeRequest> for StopPipeSvc<T> {
                        type Response = super::StopPipeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopPipeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop_pipe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopPipeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/DeletePipe" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePipeSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::DeletePipeRequest> for DeletePipeSvc<T> {
                        type Response = super::DeletePipeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePipeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_pipe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePipeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/daemon.Daemon/ListPipe" => {
                    #[allow(non_camel_case_types)]
                    struct ListPipeSvc<T: Daemon>(pub Arc<T>);
                    impl<T: Daemon> tonic::server::UnaryService<super::ListPipeRequest> for ListPipeSvc<T> {
                        type Response = super::ListPipeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPipeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_pipe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPipeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Daemon> Clone for DaemonServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Daemon> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Daemon> tonic::transport::NamedService for DaemonServer<T> {
        const NAME: &'static str = "daemon.Daemon";
    }
}
