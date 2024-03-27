pub fn route(r: http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes> {
    use ft_sdk::Layout;

    match r.uri().path() {
        // site
        "/wasm-test/" => Dummy::page::<Hello>(r),
        t => ft_sdk::not_found!("no route for {t}"),
    }
}

struct Dummy {}

impl ft_sdk::Layout for Dummy {
    type Error = ft_sdk::Error;

    fn from_in(_in_: ft_sdk::In, _ty: ft_sdk::RequestType) -> Result<Self, Self::Error> {
        ft_sdk::println!("from_in 1");

        Ok(Self {})
    }

    fn json(&mut self, page: serde_json::Value) -> Result<serde_json::Value, Self::Error> {
        Ok(serde_json::json!({"page": page}))
    }

    fn render_error(e: Self::Error) -> http::Response<bytes::Bytes> {
        ft_sdk::println!("rendering error: {e:?}");
        ft_sdk::json_response(serde_json::json!({
            "error": e.to_string(),
        }))
    }
}

#[derive(serde::Serialize)]
pub struct Hello {
    msg: String,
}

impl ft_sdk::Page<Dummy, ft_sdk::Error> for Hello {
    fn page(_i: &mut Dummy) -> Result<Self, ft_sdk::Error> {
        ft_sdk::println!("hello wasm");
        Ok(Hello {
            msg: "hello from ft_sdk".into(),
        })
    }
}
