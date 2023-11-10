//! Types for the IBC events emitted from Tendermint Websocket by the client module.
use derive_more::From;
use subtle_encoding::hex;
use tendermint::abci;

use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::height::Height;
use crate::core::ics24_host::identifier::ClientId;
use crate::prelude::*;

/// Client event types
const CREATE_CLIENT_EVENT: &str = "create_client";
const UPDATE_CLIENT_EVENT: &str = "update_client";
const CLIENT_MISBEHAVIOUR_EVENT: &str = "client_misbehaviour";
const UPGRADE_CLIENT_EVENT: &str = "upgrade_client";

/// The content of the `key` field for the attribute containing the client identifier.
pub const CLIENT_ID_ATTRIBUTE_KEY: &str = "client_id";

/// The content of the `key` field for the attribute containing the client type.
pub const CLIENT_TYPE_ATTRIBUTE_KEY: &str = "client_type";

/// The content of the `key` field for the attribute containing the height.
pub const CONSENSUS_HEIGHT_ATTRIBUTE_KEY: &str = "consensus_height";

/// The content of the `key` field for the attribute containing the heights of consensus states that were processed.
pub const CONSENSUS_HEIGHTS_ATTRIBUTE_KEY: &str = "consensus_heights";

/// The content of the `key` field for the header in update client event.
pub const HEADER_ATTRIBUTE_KEY: &str = "header";

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, From, PartialEq, Eq)]
struct ClientIdAttribute {
    client_id: ClientId,
}

impl From<ClientIdAttribute> for abci::EventAttribute {
    fn from(attr: ClientIdAttribute) -> Self {
        (CLIENT_ID_ATTRIBUTE_KEY, attr.client_id.as_str()).into()
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, From, PartialEq, Eq)]
struct ClientTypeAttribute {
    client_type: ClientType,
}

impl From<ClientTypeAttribute> for abci::EventAttribute {
    fn from(attr: ClientTypeAttribute) -> Self {
        (CLIENT_TYPE_ATTRIBUTE_KEY, attr.client_type.as_str()).into()
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, From, PartialEq, Eq)]
struct ConsensusHeightAttribute {
    consensus_height: Height,
}

impl From<ConsensusHeightAttribute> for abci::EventAttribute {
    fn from(attr: ConsensusHeightAttribute) -> Self {
        (CONSENSUS_HEIGHT_ATTRIBUTE_KEY, attr.consensus_height).into()
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, From, PartialEq, Eq)]
struct ConsensusHeightsAttribute {
    consensus_heights: Vec<Height>,
}

impl From<ConsensusHeightsAttribute> for abci::EventAttribute {
    fn from(attr: ConsensusHeightsAttribute) -> Self {
        let consensus_heights: Vec<String> = attr
            .consensus_heights
            .into_iter()
            .map(|consensus_height| consensus_height.to_string())
            .collect();
        (CONSENSUS_HEIGHTS_ATTRIBUTE_KEY, consensus_heights.join(",")).into()
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, From, PartialEq, Eq)]
struct HeaderAttribute {
    /// NOTE: The header is encoded as bytes of the
    /// [`Any`](ibc_proto::google::protobuf::Any) type.
    header: Vec<u8>,
}

impl From<HeaderAttribute> for abci::EventAttribute {
    fn from(attr: HeaderAttribute) -> Self {
        (
            HEADER_ATTRIBUTE_KEY,
            String::from_utf8(hex::encode(attr.header))
                .expect("Never fails because hexadecimal is valid UTF-8"),
        )
            .into()
    }
}

/// CreateClient event signals the creation of a new on-chain client (IBC client).
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateClient {
    client_id: ClientIdAttribute,
    client_type: ClientTypeAttribute,
    consensus_height: ConsensusHeightAttribute,
}

impl CreateClient {
    pub fn new(client_id: ClientId, client_type: ClientType, consensus_height: Height) -> Self {
        Self {
            client_id: ClientIdAttribute::from(client_id),
            client_type: ClientTypeAttribute::from(client_type),
            consensus_height: ConsensusHeightAttribute::from(consensus_height),
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id.client_id
    }

    pub fn client_type(&self) -> &ClientType {
        &self.client_type.client_type
    }

    pub fn consensus_height(&self) -> &Height {
        &self.consensus_height.consensus_height
    }

    pub fn event_type(&self) -> &str {
        CREATE_CLIENT_EVENT
    }
}

impl From<CreateClient> for abci::Event {
    fn from(c: CreateClient) -> Self {
        Self {
            kind: CREATE_CLIENT_EVENT.to_owned(),
            attributes: vec![
                c.client_id.into(),
                c.client_type.into(),
                c.consensus_height.into(),
            ],
        }
    }
}

/// UpdateClient event signals a recent update of an on-chain client (IBC Client).
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateClient {
    client_id: ClientIdAttribute,
    client_type: ClientTypeAttribute,
    // Deprecated: consensus_height is deprecated and will be removed in a future release.
    // Please use consensus_heights instead.
    consensus_height: ConsensusHeightAttribute,
    consensus_heights: ConsensusHeightsAttribute,
    header: HeaderAttribute,
}

impl UpdateClient {
    /// Constructs a new UpdateClient event.
    ///
    /// NOTE: the `header` is the encoded bytes of the
    /// [`Any`](ibc_proto::google::protobuf::Any) type.
    pub fn new(
        client_id: ClientId,
        client_type: ClientType,
        consensus_height: Height,
        consensus_heights: Vec<Height>,
        header: Vec<u8>,
    ) -> Self {
        Self {
            client_id: ClientIdAttribute::from(client_id),
            client_type: ClientTypeAttribute::from(client_type),
            consensus_height: ConsensusHeightAttribute::from(consensus_height),
            consensus_heights: ConsensusHeightsAttribute::from(consensus_heights),
            header: HeaderAttribute::from(header),
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id.client_id
    }

    pub fn client_type(&self) -> &ClientType {
        &self.client_type.client_type
    }

    pub fn consensus_height(&self) -> &Height {
        &self.consensus_height.consensus_height
    }

    pub fn consensus_heights(&self) -> &[Height] {
        self.consensus_heights.consensus_heights.as_ref()
    }

    pub fn header(&self) -> &Vec<u8> {
        &self.header.header
    }

    pub fn event_type(&self) -> &str {
        UPDATE_CLIENT_EVENT
    }
}

impl From<UpdateClient> for abci::Event {
    fn from(u: UpdateClient) -> Self {
        Self {
            kind: UPDATE_CLIENT_EVENT.to_owned(),
            attributes: vec![
                u.client_id.into(),
                u.client_type.into(),
                u.consensus_height.into(),
                u.consensus_heights.into(),
                u.header.into(),
            ],
        }
    }
}

/// ClientMisbehaviour event signals the update of an on-chain client (IBC Client) with evidence of
/// misbehaviour.
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientMisbehaviour {
    client_id: ClientIdAttribute,
    client_type: ClientTypeAttribute,
}

impl ClientMisbehaviour {
    pub fn new(client_id: ClientId, client_type: ClientType) -> Self {
        Self {
            client_id: ClientIdAttribute::from(client_id),
            client_type: ClientTypeAttribute::from(client_type),
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id.client_id
    }

    pub fn client_type(&self) -> &ClientType {
        &self.client_type.client_type
    }

    pub fn event_type(&self) -> &str {
        CLIENT_MISBEHAVIOUR_EVENT
    }
}

impl From<ClientMisbehaviour> for abci::Event {
    fn from(c: ClientMisbehaviour) -> Self {
        Self {
            kind: CLIENT_MISBEHAVIOUR_EVENT.to_owned(),
            attributes: vec![c.client_id.into(), c.client_type.into()],
        }
    }
}

/// Signals a recent upgrade of an on-chain client (IBC Client).
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpgradeClient {
    client_id: ClientIdAttribute,
    client_type: ClientTypeAttribute,
    consensus_height: ConsensusHeightAttribute,
}

impl UpgradeClient {
    pub fn new(client_id: ClientId, client_type: ClientType, consensus_height: Height) -> Self {
        Self {
            client_id: ClientIdAttribute::from(client_id),
            client_type: ClientTypeAttribute::from(client_type),
            consensus_height: ConsensusHeightAttribute::from(consensus_height),
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id.client_id
    }

    pub fn client_type(&self) -> &ClientType {
        &self.client_type.client_type
    }

    pub fn consensus_height(&self) -> &Height {
        &self.consensus_height.consensus_height
    }

    pub fn event_type(&self) -> &str {
        UPGRADE_CLIENT_EVENT
    }
}

impl From<UpgradeClient> for abci::Event {
    fn from(u: UpgradeClient) -> Self {
        Self {
            kind: UPGRADE_CLIENT_EVENT.to_owned(),
            attributes: vec![
                u.client_id.into(),
                u.client_type.into(),
                u.consensus_height.into(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ibc_proto::google::protobuf::Any;
    use ibc_testkit::utils::dummies::clients::mock::dummy_new_mock_header;
    use prost::Message;
    use tendermint::abci::Event as AbciEvent;

    use super::*;

    #[test]
    fn ibc_to_abci_client_events() {
        struct Test {
            event_kind: &'static str,
            event: AbciEvent,
            expected_keys: Vec<&'static str>,
            expected_values: Vec<&'static str>,
        }

        let client_type = ClientType::from_str("07-tendermint")
            .expect("never fails because it's a valid client type");
        let client_id = ClientId::new(client_type.clone(), 0).unwrap();
        let consensus_height = Height::new(0, 5).unwrap();
        let consensus_heights = vec![Height::new(0, 5).unwrap(), Height::new(0, 7).unwrap()];
        let header: Any = dummy_new_mock_header(5).into();
        let expected_keys = vec![
            "client_id",
            "client_type",
            "consensus_height",
            "consensus_heights",
            "header",
        ];

        let expected_values = vec![
            "07-tendermint-0",
            "07-tendermint",
            "0-5",
            "0-5,0-7",
            "0a102f6962632e6d6f636b2e48656164657212040a021005",
        ];

        let tests: Vec<Test> = vec![
            Test {
                event_kind: CREATE_CLIENT_EVENT,
                event: CreateClient::new(client_id.clone(), client_type.clone(), consensus_height)
                    .into(),
                expected_keys: expected_keys[0..3].to_vec(),
                expected_values: expected_values[0..3].to_vec(),
            },
            Test {
                event_kind: UPDATE_CLIENT_EVENT,
                event: UpdateClient::new(
                    client_id.clone(),
                    client_type.clone(),
                    consensus_height,
                    consensus_heights,
                    header.encode_to_vec(),
                )
                .into(),
                expected_keys: expected_keys.clone(),
                expected_values: expected_values.clone(),
            },
            Test {
                event_kind: UPGRADE_CLIENT_EVENT,
                event: UpgradeClient::new(client_id.clone(), client_type.clone(), consensus_height)
                    .into(),
                expected_keys: expected_keys[0..3].to_vec(),
                expected_values: expected_values[0..3].to_vec(),
            },
            Test {
                event_kind: CLIENT_MISBEHAVIOUR_EVENT,
                event: ClientMisbehaviour::new(client_id, client_type).into(),
                expected_keys: expected_keys[0..2].to_vec(),
                expected_values: expected_values[0..2].to_vec(),
            },
        ];

        for t in tests {
            assert_eq!(t.event.kind, t.event_kind);
            assert_eq!(t.expected_keys.len(), t.event.attributes.len());
            for (i, e) in t.event.attributes.iter().enumerate() {
                assert_eq!(
                    e.key, t.expected_keys[i],
                    "key mismatch for {:?}",
                    t.event_kind
                );
            }
            for (i, e) in t.event.attributes.iter().enumerate() {
                assert_eq!(
                    e.value, t.expected_values[i],
                    "value mismatch for {:?}",
                    t.event_kind
                );
            }
        }
    }
}
