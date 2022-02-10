pub trait Request<Args, Response> {
    fn build_response(&self, args: Args) -> Response;
}
