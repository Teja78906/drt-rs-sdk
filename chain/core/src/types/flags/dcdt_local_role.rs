use super::DcdtLocalRoleFlags;
use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

const DCDT_ROLE_NONE: &str = "";
const DCDT_ROLE_LOCAL_MINT: &str = "DCDTRoleLocalMint";
const DCDT_ROLE_LOCAL_BURN: &str = "DCDTRoleLocalBurn";
const DCDT_ROLE_NFT_CREATE: &str = "DCDTRoleNFTCreate";
const DCDT_ROLE_NFT_ADD_QUANTITY: &str = "DCDTRoleNFTAddQuantity";
const DCDT_ROLE_NFT_BURN: &str = "DCDTRoleNFTBurn";
const DCDT_ROLE_NFT_ADD_URI: &str = "DCDTRoleNFTAddURI";
const DCDT_ROLE_NFT_UPDATE_ATTRIBUTES: &str = "DCDTRoleNFTUpdateAttributes";
const DCDT_ROLE_TRANSFER: &str = "DCDTTransferRole";
const DCDT_ROLE_SET_NEW_URI: &str = "DCDTRoleSetNewURI";
const DCDT_ROLE_MODIFY_ROYALTIES: &str = "DCDTRoleModifyRoyalties";
const DCDT_ROLE_MODIFY_CREATOR: &str = "DCDTRoleModifyCreator";
const DCDT_ROLE_NFT_RECREATE: &str = "DCDTRoleNFTRecreate";
const DCDT_ROLE_NFT_UPDATE: &str = "DCDTRoleNFTUpdate";

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug, Copy)]
pub enum DcdtLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
    NftAddUri,
    NftUpdateAttributes,
    Transfer,
    SetNewUri,
    ModifyRoyalties,
    ModifyCreator,
    NftRecreate,
    NftUpdate,
}

impl DcdtLocalRole {
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::None => 0,
            Self::Mint => 1,
            Self::Burn => 2,
            Self::NftCreate => 3,
            Self::NftAddQuantity => 4,
            Self::NftBurn => 5,
            Self::NftAddUri => 6,
            Self::NftUpdateAttributes => 7,
            Self::Transfer => 8,
            Self::SetNewUri => 9,
            Self::ModifyRoyalties => 10,
            Self::ModifyCreator => 11,
            Self::NftRecreate => 12,
            Self::NftUpdate => 13,
        }
    }

    pub fn as_role_name(&self) -> &'static [u8] {
        self.name().as_bytes()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => DCDT_ROLE_NONE,
            Self::Mint => DCDT_ROLE_LOCAL_MINT,
            Self::Burn => DCDT_ROLE_LOCAL_BURN,
            Self::NftCreate => DCDT_ROLE_NFT_CREATE,
            Self::NftAddQuantity => DCDT_ROLE_NFT_ADD_QUANTITY,
            Self::NftBurn => DCDT_ROLE_NFT_BURN,
            Self::NftAddUri => DCDT_ROLE_NFT_ADD_URI,
            Self::NftUpdateAttributes => DCDT_ROLE_NFT_UPDATE_ATTRIBUTES,
            Self::Transfer => DCDT_ROLE_TRANSFER,
            Self::SetNewUri => DCDT_ROLE_SET_NEW_URI,
            Self::ModifyRoyalties => DCDT_ROLE_MODIFY_ROYALTIES,
            Self::ModifyCreator => DCDT_ROLE_MODIFY_CREATOR,
            Self::NftRecreate => DCDT_ROLE_NFT_RECREATE,
            Self::NftUpdate => DCDT_ROLE_NFT_UPDATE,
        }
    }

    pub fn to_flag(&self) -> DcdtLocalRoleFlags {
        match self {
            Self::None => DcdtLocalRoleFlags::NONE,
            Self::Mint => DcdtLocalRoleFlags::MINT,
            Self::Burn => DcdtLocalRoleFlags::BURN,
            Self::NftCreate => DcdtLocalRoleFlags::NFT_CREATE,
            Self::NftAddQuantity => DcdtLocalRoleFlags::NFT_ADD_QUANTITY,
            Self::NftBurn => DcdtLocalRoleFlags::NFT_BURN,
            Self::NftAddUri => DcdtLocalRoleFlags::NFT_ADD_URI,
            Self::NftUpdateAttributes => DcdtLocalRoleFlags::NFT_UPDATE_ATTRIBUTES,
            Self::Transfer => DcdtLocalRoleFlags::TRANSFER,
            Self::SetNewUri => DcdtLocalRoleFlags::SET_NEW_URI,
            Self::ModifyRoyalties => DcdtLocalRoleFlags::MODIFY_ROYALTIES,
            Self::ModifyCreator => DcdtLocalRoleFlags::MODIFY_CREATOR,
            Self::NftRecreate => DcdtLocalRoleFlags::NFT_RECREATE,
            Self::NftUpdate => DcdtLocalRoleFlags::NFT_UPDATE,
        }
    }
}

// TODO: can be done with macros, but I didn't find a public library that does it and is no_std
// we can implement it, it's easy
const ALL_ROLES: [DcdtLocalRole; 13] = [
    DcdtLocalRole::Mint,
    DcdtLocalRole::Burn,
    DcdtLocalRole::NftCreate,
    DcdtLocalRole::NftAddQuantity,
    DcdtLocalRole::NftBurn,
    DcdtLocalRole::NftAddUri,
    DcdtLocalRole::NftUpdateAttributes,
    DcdtLocalRole::Transfer,
    DcdtLocalRole::SetNewUri,
    DcdtLocalRole::ModifyRoyalties,
    DcdtLocalRole::ModifyCreator,
    DcdtLocalRole::NftRecreate,
    DcdtLocalRole::NftUpdate,
];

impl DcdtLocalRole {
    pub fn iter_all() -> core::slice::Iter<'static, DcdtLocalRole> {
        ALL_ROLES.iter()
    }
}

impl From<u16> for DcdtLocalRole {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Mint,
            2 => Self::Burn,
            3 => Self::NftCreate,
            4 => Self::NftAddQuantity,
            5 => Self::NftBurn,
            6 => Self::NftAddUri,
            7 => Self::NftUpdateAttributes,
            8 => Self::Transfer,
            9 => Self::SetNewUri,
            10 => Self::ModifyRoyalties,
            11 => Self::ModifyCreator,
            12 => Self::NftRecreate,
            13 => Self::NftUpdate,
            _ => Self::None,
        }
    }
}

impl<'a> From<&'a [u8]> for DcdtLocalRole {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == DCDT_ROLE_LOCAL_MINT.as_bytes() {
            Self::Mint
        } else if byte_slice == DCDT_ROLE_LOCAL_BURN.as_bytes() {
            Self::Burn
        } else if byte_slice == DCDT_ROLE_NFT_CREATE.as_bytes() {
            Self::NftCreate
        } else if byte_slice == DCDT_ROLE_NFT_ADD_QUANTITY.as_bytes() {
            Self::NftAddQuantity
        } else if byte_slice == DCDT_ROLE_NFT_BURN.as_bytes() {
            Self::NftBurn
        } else if byte_slice == DCDT_ROLE_NFT_ADD_URI.as_bytes() {
            Self::NftAddUri
        } else if byte_slice == DCDT_ROLE_NFT_UPDATE_ATTRIBUTES.as_bytes() {
            Self::NftUpdateAttributes
        } else if byte_slice == DCDT_ROLE_TRANSFER.as_bytes() {
            Self::Transfer
        } else if byte_slice == DCDT_ROLE_SET_NEW_URI.as_bytes() {
            Self::SetNewUri
        } else if byte_slice == DCDT_ROLE_MODIFY_ROYALTIES.as_bytes() {
            Self::ModifyRoyalties
        } else if byte_slice == DCDT_ROLE_MODIFY_CREATOR.as_bytes() {
            Self::ModifyCreator
        } else if byte_slice == DCDT_ROLE_NFT_RECREATE.as_bytes() {
            Self::NftRecreate
        } else if byte_slice == DCDT_ROLE_NFT_UPDATE.as_bytes() {
            Self::NftUpdate
        } else {
            Self::None
        }
    }
}
