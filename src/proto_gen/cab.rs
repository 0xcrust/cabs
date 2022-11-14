/// A request to record location of a cab
/// Name:unique name for a cab
/// Location: current location of the given cab
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CabLocationRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub location: ::core::option::Option<Location>,
}
/// A request for a CabLocationRequest
/// Accepted: a boolean indicating if this request was accepted 
/// for processing
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CabLocationResponse {
    #[prost(bool, tag="1")]
    pub accepted: bool,
}
/// A request to return Cabs at a given location
/// Location:a given location
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCabRequest {
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<Location>,
}
/// A response for GetCabLocation
/// Cab: the cab gotten
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCabResponse {
    #[prost(message, repeated, tag="1")]
    pub cabs: ::prost::alloc::vec::Vec<Cab>,
}
/// Message that the CabLocationRequest passes to the server
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cab {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub location: ::core::option::Option<Location>,
}
/// Message with the location of a cab
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(float, tag="1")]
    pub latitude: f32,
    #[prost(enumeration="location::Direction", tag="2")]
    pub lat_direction: i32,
    #[prost(float, tag="3")]
    pub longitude: f32,
    #[prost(enumeration="location::Direction", tag="4")]
    pub long_direction: i32,
}
/// Nested message and enum types in `Location`.
pub mod location {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        North = 0,
        South = 1,
        East = 2,
        West = 3,
    }
}
/// Generated client implementations.
pub mod cab_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Top levelgRPC service with two RPC calls
    #[derive(Debug, Clone)]
    pub struct CabServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CabServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CabServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CabServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CabServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn record_cab_location(
            &mut self,
            request: impl tonic::IntoRequest<super::CabLocationRequest>,
        ) -> Result<tonic::Response<super::CabLocationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cab.CabService/record_cab_location",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_cabs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCabRequest>,
        ) -> Result<tonic::Response<super::GetCabResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cab.CabService/get_cabs");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod cab_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CabServiceServer.
    #[async_trait]
    pub trait CabService: Send + Sync + 'static {
        async fn record_cab_location(
            &self,
            request: tonic::Request<super::CabLocationRequest>,
        ) -> Result<tonic::Response<super::CabLocationResponse>, tonic::Status>;
        async fn get_cabs(
            &self,
            request: tonic::Request<super::GetCabRequest>,
        ) -> Result<tonic::Response<super::GetCabResponse>, tonic::Status>;
    }
    /// Top levelgRPC service with two RPC calls
    #[derive(Debug)]
    pub struct CabServiceServer<T: CabService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CabService> CabServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CabServiceServer<T>
    where
        T: CabService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/cab.CabService/record_cab_location" => {
                    #[allow(non_camel_case_types)]
                    struct record_cab_locationSvc<T: CabService>(pub Arc<T>);
                    impl<
                        T: CabService,
                    > tonic::server::UnaryService<super::CabLocationRequest>
                    for record_cab_locationSvc<T> {
                        type Response = super::CabLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CabLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).record_cab_location(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = record_cab_locationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cab.CabService/get_cabs" => {
                    #[allow(non_camel_case_types)]
                    struct get_cabsSvc<T: CabService>(pub Arc<T>);
                    impl<T: CabService> tonic::server::UnaryService<super::GetCabRequest>
                    for get_cabsSvc<T> {
                        type Response = super::GetCabResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCabRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_cabs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = get_cabsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: CabService> Clone for CabServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CabService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CabService> tonic::transport::NamedService for CabServiceServer<T> {
        const NAME: &'static str = "cab.CabService";
    }
}
