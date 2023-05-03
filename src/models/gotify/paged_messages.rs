use serde_derive::{Deserialize, Serialize};

use super::{message::MessageModel, paging::PagingModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedMessagesModel {
    pub messages: Vec<MessageModel>,
    pub paging: PagingModel,
}
