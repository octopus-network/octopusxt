use crate::primitive::{QueryDenomTraceResponse, QueryDenomTracesResponse};
use crate::{OctopusxtClient, Transfer};
use ibc::Height;

impl Transfer for OctopusxtClient {
    type Error = anyhow::Error;

    /// Query the denom trace for an ibc denom
    fn query_denom_trace(&self, _denom: String) -> Result<QueryDenomTraceResponse, Self::Error> {
        todo!()
    }

    fn query_denom_traces(
        &self,
        _offset: String,
        _limit: u64,
        _height: Height,
    ) -> Result<QueryDenomTracesResponse, Self::Error> {
        todo!()
    }
}
