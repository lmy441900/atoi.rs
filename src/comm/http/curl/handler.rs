//! Handler implementation of [curl::easy::Easy2] for [CurlHttpClient].

#[derive(Debug, Default)]
pub(super) struct CurlHttpClientHandler(pub(super) Vec<u8>);

impl curl::easy::Handler for CurlHttpClientHandler {
    fn write(&mut self, data: &[u8]) -> Result<usize, curl::easy::WriteError> {
        self.0.extend(data);
        Ok(data.len())
    }
}

impl CurlHttpClientHandler {
    pub fn new() -> Self {
        Default::default()
    }
}
