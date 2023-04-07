use anyhow::Ok;
use node_template_runtime::Runtime;
use sp_core::sr25519;
use substrate_api_client::{rpc::WsRpcClient, Api, GetBlock, GetHeader, PlainTipExtrinsicParams};

use crate::{chain::*, error_types::Error};

/// 区块链连接
#[derive(Debug, Clone)]
pub struct Client {
    // u32
    pub index: u32,

    // 链接
    api: Option<String>,
}

impl Client {
    pub fn new(uri: String) -> anyhow::Result<Self, anyhow::Error> {
        let i = get_api_index(uri.clone())?;

        Ok(Client {
            index: i,
            api: None,
        })
    }

    pub fn from_index(index: u32) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Client { index, api: None })
    }

    // pub fn get_api(&mut self) -> anyhow::Result<Api, anyhow::Error> {
    //     if self.api.is_some() {
    //         return Ok(self.api.clone().unwrap());
    //     }

    //     let apis = API_POOL.lock().unwrap();
    //     let url = apis.get(self.index as usize).unwrap();

    //     let client = WsRpcClient::new(url);
    //     let api = Api::<_, _, PlainTipExtrinsicParams<Runtime>, Runtime>::new(client).unwrap();
    //     self.api = Some(api.clone());

    //     Ok(api)
    // }

    pub fn get_url(&mut self) -> anyhow::Result<String, anyhow::Error> {
        let apis = API_POOL.lock().unwrap();
        let url = apis.get(self.index as usize).unwrap();

        Ok(url.clone())
    }

    pub async fn get_block_number(&mut self) -> Result<(u32, String), anyhow::Error> {
        // 获取区块链接口
        // let api = self.get_api().await?;
        let apis = API_POOL.lock().unwrap();
        let url = apis.get(self.index as usize).unwrap();

        let client = WsRpcClient::new(url).unwrap();
        let api = Api::<sr25519::Pair, _, PlainTipExtrinsicParams<Runtime>, Runtime>::new(client)
            .unwrap();

        let header_hash = api.get_finalized_head().unwrap().unwrap();
        let h = api.get_header(Some(header_hash)).unwrap().unwrap();

        Ok((h.number, header_hash.to_string()))
    }

    // pub async fn subscribe_block(&self) -> Result<(), subxt::Error> {
    //     // 获取区块链接口
    //     let apis = API_POOL.lock().unwrap();
    //     let api = apis.get(self.index as usize).unwrap();
    //     let mut block_sub = api.rpc().subscribe_finalized_blocks().await?;

    //     while let Some(Ok(block)) = block_sub.next().await {
    //         println!(
    //             "block number: {} hash:{} parent:{} state root:{} extrinsics root:{}",
    //             block.number,
    //             block.hash(),
    //             block.parent_hash,
    //             block.state_root,
    //             block.extrinsics_root
    //         );
    //     }

    //     Ok(())
    // }

    // pub async fn unsubscribe(&self) -> Result<(), subxt::Error> {
    //     // 获取区块链接口
    //     let apis = API_POOL.lock().unwrap();
    //     let api = apis.get(self.index as usize).unwrap();
    //     let mut block_sub = api.rpc().subscribe_finalized_blocks().await?;

    //     while let Some(Ok(block)) = block_sub.next().await {
    //         println!(
    //             "block number: {} hash:{} parent:{} state root:{} extrinsics root:{}",
    //             block.number,
    //             block.hash(),
    //             block.parent_hash,
    //             block.state_root,
    //             block.extrinsics_root
    //         );
    //     }

    //     Ok(())
    // }
}
