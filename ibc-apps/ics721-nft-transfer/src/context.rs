//! Defines the required context traits for ICS-721 to interact with host
//! machine.
use ibc_core::host::types::identifiers::{ChannelId, PortId};
use ibc_core::primitives::prelude::*;
use ibc_core::primitives::Signer;

use crate::types::error::NftTransferError;
use crate::types::{
    ClassData, ClassId, ClassUri, Memo, PrefixedClassId, TokenData, TokenId, TokenUri,
};

pub trait NftContext {
    /// Get the class ID of the token
    fn get_class_id(&self) -> &ClassId;

    /// Get the token ID
    fn get_id(&self) -> &TokenId;

    /// Get the token URI
    fn get_uri(&self) -> &TokenUri;

    /// Get the token Data
    fn get_data(&self) -> &TokenData;
}

pub trait NftClassContext {
    /// Get the class ID
    fn get_id(&self) -> &ClassId;

    /// Get the class URI
    fn get_uri(&self) -> &ClassUri;

    /// Get the class Data
    fn get_data(&self) -> &ClassData;
}

/// Read-only methods required in NFT transfer validation context.
pub trait NftTransferValidationContext {
    type AccountId: TryFrom<Signer> + PartialEq;
    type Nft: NftContext;
    type NftClass: NftClassContext;

    /// get_port returns the portID for the transfer module.
    fn get_port(&self) -> Result<PortId, NftTransferError>;

    /// Returns Ok() if the host chain supports sending NFTs.
    fn can_send_nft(&self) -> Result<(), NftTransferError>;

    /// Returns Ok() if the host chain supports receiving NFTs.
    fn can_receive_nft(&self) -> Result<(), NftTransferError>;

    /// Validates that the NFT can be created or updated successfully.
    fn create_or_update_class_validate(
        &self,
        class_id: &PrefixedClassId,
        class_uri: &ClassUri,
        class_data: &ClassData,
    ) -> Result<(), NftTransferError>;

    /// Validates that the tokens can be escrowed successfully.
    ///
    /// The owner of the NFT should be checked in this validation.
    /// `memo` field allows to incorporate additional contextual details in the
    /// escrow validation.
    fn escrow_nft_validate(
        &self,
        from_account: &Self::AccountId,
        port_id: &PortId,
        channel_id: &ChannelId,
        class_id: &PrefixedClassId,
        token_id: &TokenId,
        memo: &Memo,
    ) -> Result<(), NftTransferError>;

    /// Validates that the NFT can be unescrowed successfully.
    fn unescrow_nft_validate(
        &self,
        to_account: &Self::AccountId,
        port_id: &PortId,
        channel_id: &ChannelId,
        class_id: &PrefixedClassId,
        token_id: &TokenId,
    ) -> Result<(), NftTransferError>;

    /// Validates the receiver account and the NFT input
    fn mint_nft_validate(
        &self,
        account: &Self::AccountId,
        class_id: &PrefixedClassId,
        token_id: &TokenId,
        token_uri: &TokenUri,
        token_data: &TokenData,
    ) -> Result<(), NftTransferError>;

    /// Validates the sender account and the coin input before burning.
    ///
    /// The owner of the NFT should be checked in this validation.
    /// `memo` field allows to incorporate additional contextual details in the
    /// burn validation.
    fn burn_nft_validate(
        &self,
        account: &Self::AccountId,
        class_id: &PrefixedClassId,
        token_id: &TokenId,
        memo: &Memo,
    ) -> Result<(), NftTransferError>;

    /// Returns a hash of the prefixed class ID.
    /// Implement only if the host chain supports hashed class ID.
    fn class_hash_string(&self, _class_id: &PrefixedClassId) -> Option<String> {
        None
    }

    /// Returns the NFT
    fn get_nft(
        &self,
        class_id: &PrefixedClassId,
        token_id: &TokenId,
    ) -> Result<Self::Nft, NftTransferError>;

    /// Returns the NFT class
    fn get_nft_class(&self, class_id: &PrefixedClassId)
        -> Result<Self::NftClass, NftTransferError>;
}

/// Read-write methods required in NFT transfer execution context.
pub trait NftTransferExecutionContext: NftTransferValidationContext {
    /// Creates a new NFT Class identified by classId. If the class ID already exists, it updates the class metadata.
    fn create_or_update_class_execute(
        &self,
        class_id: &PrefixedClassId,
        class_uri: &ClassUri,
        class_data: &ClassData,
    ) -> Result<(), NftTransferError>;

    /// Executes the escrow of the NFT in a user account.
    ///
    /// `memo` field allows to incorporate additional contextual details in the
    /// escrow execution.
    fn escrow_nft_execute(
        &mut self,
        from_account: &Self::AccountId,
        port_id: &PortId,
        channel_id: &ChannelId,
        class_id: &PrefixedClassId,
        token_id: &TokenId,
        memo: &Memo,
    ) -> Result<(), NftTransferError>;

    /// Executes the unescrow of the NFT in a user account.
    fn unescrow_nft_execute(
        &mut self,
        to_account: &Self::AccountId,
        port_id: &PortId,
        channel_id: &ChannelId,
        class_id: &PrefixedClassId,
        token_id: &TokenId,
    ) -> Result<(), NftTransferError>;

    /// Executes minting of the NFT in a user account.
    fn mint_nft_execute(
        &mut self,
        account: &Self::AccountId,
        class_id: &PrefixedClassId,
        token_id: &TokenId,
        token_uri: &TokenUri,
        token_data: &TokenData,
    ) -> Result<(), NftTransferError>;

    /// Executes burning of the NFT in a user account.
    ///
    /// `memo` field allows to incorporate additional contextual details in the
    /// burn execution.
    fn burn_nft_execute(
        &mut self,
        account: &Self::AccountId,
        class_id: &PrefixedClassId,
        token_id: &TokenId,
        memo: &Memo,
    ) -> Result<(), NftTransferError>;
}
