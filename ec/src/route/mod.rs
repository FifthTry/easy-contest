pub mod contest;

pub fn route(r: http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes> {
    use ft_sdk::Layout;

    match r.uri().path() {
        "/ft2/contest/submissions/" => contest::Contest::page::<contest::Submissions>(r),
        "/ft2/contest/submissions/new/" => {
            contest::Contest::action::<contest::SubmissionPayload>(r)
        }
        "/ft2/contest/submissions/delete/" => {
            contest::Contest::action::<contest::DeleteSubmissionPayload>(r)
        }

        t => ft_sdk::not_found!("no route for {t}"),
    }
}
