pub mod contest;

pub fn route(r: http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes> {
    use ft_sdk::Layout;

    match r.uri().path() {
        "/ec/contest/submissions/" => contest::Contest::page::<contest::Submissions>(r),
        "/ec/contest/submissions/new/" => contest::Contest::action::<contest::SubmissionPayload>(r),
        "/ec/contest/submissions/delete/" => {
            contest::Contest::action::<contest::DeleteSubmissionPayload>(r)
        }

        t => ft_sdk::not_found!("no route for {t}"),
    }
}
