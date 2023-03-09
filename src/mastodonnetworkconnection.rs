mod mastodonNetworkConnection {
    mod mastodonconnection {
        pub mod request {
            pub struct Request {
                path: String,
                query_string: Option<String>,
                method: super::httpMethod::HttpMethod,
            }
        }

        pub mod httpMethod {
            pub enum HttpMethod {
                GET,
                DELETE,
                POST,
                PUT,
                HEAD,
                CONNECT,
                OPTIONS,
                TRACE,
                PATCH,
            }
        }
    }
}
