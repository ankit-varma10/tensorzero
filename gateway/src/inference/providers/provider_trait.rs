use crate::error::Error;
use crate::inference::types::InferenceResponseStream;
use crate::inference::types::ModelInferenceRequest;
use crate::inference::types::ModelInferenceResponse;
use crate::inference::types::ModelInferenceResponseChunk;
use futures::Future;
use reqwest::Client;

pub trait InferenceProvider {
    fn infer<'a>(
        &'a self,
        request: &'a ModelInferenceRequest,
        client: &'a Client,
    ) -> impl Future<Output = Result<ModelInferenceResponse, Error>> + Send + 'a;

    fn infer_stream<'a>(
        &'a self,
        request: &'a ModelInferenceRequest,
        client: &'a Client,
    ) -> impl Future<Output = Result<(ModelInferenceResponseChunk, InferenceResponseStream), Error>>
           + Send
           + 'a;
}