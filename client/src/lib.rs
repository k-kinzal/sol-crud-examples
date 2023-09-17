mod instruction;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::de::DeserializeOwned;
use serde::Serialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::{declare_id, system_instruction};
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};
use std::marker::PhantomData;

declare_id!("59j4t3Gwow8FN1TV5F4Rm8GuG7eXc2y4WStzbZ1AJ8Ro");
// declare_id!("9HsszdXvbSBsEHpn6GXAFgaaH2qLTbpZpc7pPrt4RAyF");

pub struct Client {
    rpc: RpcClient,
    keypair: Keypair,
}

impl Client {
    pub fn new(json_rpc_url: String, keypair: Keypair) -> Self {
        Self {
            rpc: RpcClient::new(json_rpc_url),
            keypair,
        }
    }

    pub async fn create(
        &self,
        bytes: &[u8],
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        let program_id = id();

        let state = Keypair::new();
        let lamports = self
            .rpc
            .get_minimum_balance_for_rent_exemption(bytes.len())
            .await?;
        let blockhash = self.rpc.get_latest_blockhash().await?;
        let transaction = Transaction::new_signed_with_payer(
            &[
                system_instruction::create_account(
                    &self.keypair.pubkey(),
                    &state.pubkey(),
                    lamports,
                    bytes.len() as u64,
                    &program_id,
                ),
                instruction::create_data(program_id, state.pubkey(), bytes)?,
            ],
            Some(&self.keypair.pubkey()),
            &[&self.keypair, &state],
            blockhash,
        );

        let sig = self.rpc.send_and_confirm_transaction(&transaction).await?;
        let res = self
            .rpc
            .get_transaction(&sig, UiTransactionEncoding::JsonParsed)
            .await?;

        Ok(res)
    }

    pub async fn update(
        &self,
        key: Pubkey,
        data: &[u8],
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        let program_id = id();

        let blockhash = self.rpc.get_latest_blockhash().await?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction::update_data(program_id, key, data)?],
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            blockhash,
        );

        let sig = self.rpc.send_and_confirm_transaction(&transaction).await;
        println!("{:?}", sig);
        let sig = sig?;
        let res = self
            .rpc
            .get_transaction(&sig, UiTransactionEncoding::JsonParsed)
            .await?;

        Ok(res)
    }

    pub async fn delete(
        &self,
        key: Pubkey,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        let program_id = id();

        let blockhash = self.rpc.get_latest_blockhash().await?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction::delete_data(
                program_id,
                key,
                self.keypair.pubkey(),
            )?],
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            blockhash,
        );

        let sig = self.rpc.send_and_confirm_transaction(&transaction).await;
        println!("{:?}", sig);
        let sig = sig?;
        let res = self
            .rpc
            .get_transaction(&sig, UiTransactionEncoding::JsonParsed)
            .await?;

        Ok(res)
    }

    pub async fn get(&self, key: Pubkey) -> anyhow::Result<Vec<u8>> {
        let account = self.rpc.get_account_data(&key).await?;
        Ok(account)
    }
}

pub struct BorshClient<T>(Client, PhantomData<T>);

impl<T> BorshClient<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    pub fn new(json_rpc_url: String, keypair: Keypair) -> Self {
        Self(Client::new(json_rpc_url, keypair), PhantomData)
    }

    pub async fn create(
        &self,
        data: T,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        let bytes = data.try_to_vec()?;
        self.0.create(&bytes).await
    }

    pub async fn update(
        &self,
        key: Pubkey,
        data: T,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        let bytes = data.try_to_vec()?;
        self.0.update(key, &bytes).await
    }

    pub async fn delete(
        &self,
        key: Pubkey,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        self.0.delete(key).await
    }

    pub async fn get(&self, key: Pubkey) -> anyhow::Result<T> {
        let bytes = self.0.get(key).await?;
        let data = T::try_from_slice(&bytes)?;
        Ok(data)
    }
}

pub struct JsonClient<T>(Client, PhantomData<T>);

impl<T> JsonClient<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(json_rpc_url: String, keypair: Keypair) -> Self {
        Self(Client::new(json_rpc_url, keypair), PhantomData)
    }

    pub async fn create(
        &self,
        data: T,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        let bytes = serde_json::to_vec(&data)?;
        self.0.create(&bytes).await
    }

    pub async fn update(
        &self,
        key: Pubkey,
        data: T,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        let bytes = serde_json::to_vec(&data)?;
        self.0.update(key, &bytes).await
    }

    pub async fn delete(
        &self,
        key: Pubkey,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        self.0.delete(key).await
    }

    pub async fn get(&self, key: Pubkey) -> anyhow::Result<T> {
        let bytes = self.0.get(key).await?;
        let data = serde_json::from_slice(&bytes)?;
        Ok(data)
    }
}
