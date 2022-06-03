use crate::*;


#[derive(Deserialize, Serialize, Default)]
#[serde(crate = "near_sdk::serde")]
#[witgen]
pub struct InitialMetadata {
    name: String,
    symbol: String,
    uri: String,
    icon: Option<String>,
    spec: Option<String>,
    reference: Option<String>,
    reference_hash: Option<Base64VecU8>,
}

impl From<InitialMetadata> for NFTContractMetadata {
    fn from(initial_metadata: InitialMetadata) -> Self {
        let InitialMetadata {
            spec,
            name,
            symbol,
            icon,
            uri,
            reference,
            reference_hash,
        } = initial_metadata;
        NFTContractMetadata {
            spec: spec.unwrap_or_else(|| NFT_METADATA_SPEC.to_string()),
            name,
            symbol,
            icon,
            base_uri: Some(uri),
            reference,
            reference_hash,
        }
    }
}
