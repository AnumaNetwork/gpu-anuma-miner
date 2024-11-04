use crate::proto::{
    anumad_message::Payload, GetBlockTemplateRequestMessage, GetInfoRequestMessage, AnumadMessage,
    NotifyBlockAddedRequestMessage, NotifyNewBlockTemplateRequestMessage, RpcBlock, SubmitBlockRequestMessage,
};
use crate::{
    pow::{self, HeaderHasher},
    Hash,
};

impl AnumadMessage {
    #[inline(always)]
    pub fn get_info_request() -> Self {
        AnumadMessage { payload: Some(Payload::GetInfoRequest(GetInfoRequestMessage {})) }
    }
    #[inline(always)]
    pub fn notify_block_added() -> Self {
        AnumadMessage { payload: Some(Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage {})) }
    }

    #[inline(always)]
    pub fn submit_block(block: RpcBlock) -> Self {
        AnumadMessage {
            payload: Some(Payload::SubmitBlockRequest(SubmitBlockRequestMessage {
                block: Some(block),
                allow_non_daa_blocks: false,
            })),
        }
    }
}

impl From<GetInfoRequestMessage> for AnumadMessage {
    fn from(a: GetInfoRequestMessage) -> Self {
        AnumadMessage { payload: Some(Payload::GetInfoRequest(a)) }
    }
}
impl From<NotifyBlockAddedRequestMessage> for AnumadMessage {
    fn from(a: NotifyBlockAddedRequestMessage) -> Self {
        AnumadMessage { payload: Some(Payload::NotifyBlockAddedRequest(a)) }
    }
}

impl From<GetBlockTemplateRequestMessage> for AnumadMessage {
    fn from(a: GetBlockTemplateRequestMessage) -> Self {
        AnumadMessage { payload: Some(Payload::GetBlockTemplateRequest(a)) }
    }
}

impl From<NotifyNewBlockTemplateRequestMessage> for AnumadMessage {
    fn from(a: NotifyNewBlockTemplateRequestMessage) -> Self {
        AnumadMessage { payload: Some(Payload::NotifyNewBlockTemplateRequest(a)) }
    }
}

impl RpcBlock {
    #[inline(always)]
    pub fn block_hash(&self) -> Option<Hash> {
        let mut hasher = HeaderHasher::new();
        pow::serialize_header(&mut hasher, self.header.as_ref()?, false);
        Some(hasher.finalize())
    }
}
