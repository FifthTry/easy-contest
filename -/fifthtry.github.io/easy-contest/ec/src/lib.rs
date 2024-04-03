extern crate self as ec;

mod route;
mod schema;
mod urls;

pub use route::contest::Contest;
pub use route::contest::ContestError;

#[no_mangle]
pub extern "C" fn main_ft() {
    let req = ft_sdk::http::current_request();
    let resp = route::route(req);
    ft_sdk::http::send_response(resp);
}
