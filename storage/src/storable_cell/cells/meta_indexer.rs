use borsh::{BorshDeserialize, BorshSerialize};
use nssa::V03State;

use crate::{
    CF_ACC_META, CF_BREAKPOINT_NAME, CF_HASH_TO_ID, CF_META_NAME, CF_TX_TO_ID,
    DB_META_LAST_BREAKPOINT_ID, DB_META_LAST_OBSERVED_L1_LIB_HEADER_ID_IN_DB_KEY, DbResult,
    error::DbError,
    storable_cell::{SimpleReadableCell, SimpleStorableCell, SimpleWritableCell},
};

#[derive(Debug)]
pub struct LastObservedL1LibHeaderCell(pub [u8; 32]);

impl BorshSerialize for LastObservedL1LibHeaderCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        <[u8; 32]>::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for LastObservedL1LibHeaderCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        <[u8; 32]>::deserialize_reader(reader).map(LastObservedL1LibHeaderCell)
    }
}

impl SimpleStorableCell for LastObservedL1LibHeaderCell {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_META_LAST_OBSERVED_L1_LIB_HEADER_ID_IN_DB_KEY;
    const CF_NAME: &'static str = CF_META_NAME;

    fn key_constructor(_params: Self::KeyParams) -> DbResult<Vec<u8>> {
        borsh::to_vec(&Self::CELL_NAME).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some(format!("Failed to serialize {:?}", Self::CELL_NAME)),
            )
        })
    }

    fn value_constructor(&self) -> DbResult<Vec<u8>> {
        borsh::to_vec(&self).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some("Failed to serialize last observed l1 header".to_owned()),
            )
        })
    }
}

impl SimpleReadableCell for LastObservedL1LibHeaderCell {}

impl SimpleWritableCell for LastObservedL1LibHeaderCell {}

#[derive(Debug)]
pub struct LastBreakpointIdCell(pub u64);

impl BorshSerialize for LastBreakpointIdCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        u64::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for LastBreakpointIdCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        u64::deserialize_reader(reader).map(LastBreakpointIdCell)
    }
}

impl SimpleStorableCell for LastBreakpointIdCell {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_META_LAST_BREAKPOINT_ID;
    const CF_NAME: &'static str = CF_META_NAME;

    fn key_constructor(_params: Self::KeyParams) -> DbResult<Vec<u8>> {
        borsh::to_vec(&Self::CELL_NAME).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some(format!("Failed to serialize {:?}", Self::CELL_NAME)),
            )
        })
    }

    fn value_constructor(&self) -> DbResult<Vec<u8>> {
        borsh::to_vec(&self).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some("Failed to serialize last breakpoint id".to_owned()),
            )
        })
    }
}

impl SimpleReadableCell for LastBreakpointIdCell {}

impl SimpleWritableCell for LastBreakpointIdCell {}

pub struct BreakpointCellOwned(pub V03State);

impl BorshDeserialize for BreakpointCellOwned {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        V03State::deserialize_reader(reader).map(BreakpointCellOwned)
    }
}

impl SimpleStorableCell for BreakpointCellOwned {
    type KeyParams = u64;

    const CELL_NAME: &'static str = "breakpoint";
    const CF_NAME: &'static str = CF_BREAKPOINT_NAME;

    fn key_constructor(params: Self::KeyParams) -> DbResult<Vec<u8>> {
        borsh::to_vec(&params).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some(format!("Failed to serialize {:?}", Self::CELL_NAME)),
            )
        })
    }

    fn value_constructor(&self) -> DbResult<Vec<u8>> {
        borsh::to_vec(&self.0).map_err(|err| {
            DbError::borsh_cast_message(err, Some("Failed to serialize breakpoint".to_owned()))
        })
    }
}

impl SimpleReadableCell for BreakpointCellOwned {}

pub struct BreakpointCellRef<'state>(pub &'state V03State);

impl BorshSerialize for BreakpointCellRef<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        V03State::serialize(self.0, writer)
    }
}

impl SimpleStorableCell for BreakpointCellRef<'_> {
    type KeyParams = u64;

    const CELL_NAME: &'static str = "breakpoint";
    const CF_NAME: &'static str = CF_BREAKPOINT_NAME;

    fn key_constructor(params: Self::KeyParams) -> DbResult<Vec<u8>> {
        borsh::to_vec(&params).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some(format!("Failed to serialize {:?}", Self::CELL_NAME)),
            )
        })
    }

    fn value_constructor(&self) -> DbResult<Vec<u8>> {
        borsh::to_vec(&self).map_err(|err| {
            DbError::borsh_cast_message(err, Some("Failed to serialize breakpoint".to_owned()))
        })
    }
}

impl SimpleWritableCell for BreakpointCellRef<'_> {}

#[derive(Debug)]
pub struct BlockHashToBlockIdMapCell(pub u64);

impl BorshSerialize for BlockHashToBlockIdMapCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        u64::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for BlockHashToBlockIdMapCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        u64::deserialize_reader(reader).map(BlockHashToBlockIdMapCell)
    }
}

impl SimpleStorableCell for BlockHashToBlockIdMapCell {
    type KeyParams = [u8; 32];

    const CELL_NAME: &'static str = "block hash";
    const CF_NAME: &'static str = CF_HASH_TO_ID;

    fn key_constructor(params: Self::KeyParams) -> DbResult<Vec<u8>> {
        borsh::to_vec(&params).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some(format!("Failed to serialize {:?}", Self::CELL_NAME)),
            )
        })
    }

    fn value_constructor(&self) -> DbResult<Vec<u8>> {
        borsh::to_vec(&self).map_err(|err| {
            DbError::borsh_cast_message(err, Some("Failed to serialize block id".to_owned()))
        })
    }
}

impl SimpleReadableCell for BlockHashToBlockIdMapCell {}

impl SimpleWritableCell for BlockHashToBlockIdMapCell {}

#[derive(Debug)]
pub struct TxHashToBlockIdMapCell(pub u64);

impl BorshSerialize for TxHashToBlockIdMapCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        u64::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for TxHashToBlockIdMapCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        u64::deserialize_reader(reader).map(TxHashToBlockIdMapCell)
    }
}

impl SimpleStorableCell for TxHashToBlockIdMapCell {
    type KeyParams = [u8; 32];

    const CELL_NAME: &'static str = "tx hash";
    const CF_NAME: &'static str = CF_TX_TO_ID;

    fn key_constructor(params: Self::KeyParams) -> DbResult<Vec<u8>> {
        borsh::to_vec(&params).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some(format!("Failed to serialize {:?}", Self::CELL_NAME)),
            )
        })
    }

    fn value_constructor(&self) -> DbResult<Vec<u8>> {
        borsh::to_vec(&self).map_err(|err| {
            DbError::borsh_cast_message(err, Some("Failed to serialize block id".to_owned()))
        })
    }
}

impl SimpleReadableCell for TxHashToBlockIdMapCell {}

impl SimpleWritableCell for TxHashToBlockIdMapCell {}

#[derive(Debug)]
pub struct AccNumTxCell(pub u64);

impl BorshSerialize for AccNumTxCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        u64::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for AccNumTxCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        u64::deserialize_reader(reader).map(AccNumTxCell)
    }
}

impl SimpleStorableCell for AccNumTxCell {
    type KeyParams = [u8; 32];

    const CELL_NAME: &'static str = "acc id";
    const CF_NAME: &'static str = CF_ACC_META;

    fn key_constructor(params: Self::KeyParams) -> DbResult<Vec<u8>> {
        borsh::to_vec(&params).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some(format!("Failed to serialize {:?}", Self::CELL_NAME)),
            )
        })
    }

    fn value_constructor(&self) -> DbResult<Vec<u8>> {
        borsh::to_vec(&self).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some("Failed to serialize number of transactions".to_owned()),
            )
        })
    }
}

impl SimpleReadableCell for AccNumTxCell {}

impl SimpleWritableCell for AccNumTxCell {}
