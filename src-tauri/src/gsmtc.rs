use tokio::sync::mpsc::UnboundedSender;

use crate::UpdateType;

pub struct GSMTCManager {

}

pub async fn init(tx: &mut UnboundedSender<UpdateType>) {
}

impl GSMTCManager {
    pub fn new() -> GSMTCManager {


        GSMTCManager {  }
    }   
}