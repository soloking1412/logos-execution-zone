use borsh::{BorshDeserialize, BorshSerialize};
use common::block::BlockMeta;
use nssa::V03State;

use crate::{
    CF_META_NAME, CF_NSSA_STATE_NAME, DB_META_LAST_FINALIZED_BLOCK_ID,
    DB_META_LATEST_BLOCK_META_KEY, DB_NSSA_STATE_KEY, DbResult,
    error::DbError,
    storable_cell::{SimpleReadableCell, SimpleStorableCell, SimpleWritableCell},
};

pub struct NSSAStateCellOwned(pub V03State);

impl BorshDeserialize for NSSAStateCellOwned {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        V03State::deserialize_reader(reader).map(NSSAStateCellOwned)
    }
}

impl SimpleStorableCell for NSSAStateCellOwned {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_NSSA_STATE_KEY;
    const CF_NAME: &'static str = CF_NSSA_STATE_NAME;

    fn key_constructor(_params: Self::KeyParams) -> DbResult<Vec<u8>> {
        borsh::to_vec(&Self::CELL_NAME).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some(format!("Failed to serialize {:?}", Self::CELL_NAME)),
            )
        })
    }

    fn value_constructor(&self) -> DbResult<Vec<u8>> {
        borsh::to_vec(&self.0).map_err(|err| {
            DbError::borsh_cast_message(err, Some("Failed to serialize last state".to_owned()))
        })
    }
}

impl SimpleReadableCell for NSSAStateCellOwned {}

pub struct NSSAStateCellRef<'state>(pub &'state V03State);

impl BorshSerialize for NSSAStateCellRef<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        V03State::serialize(self.0, writer)
    }
}

impl SimpleStorableCell for NSSAStateCellRef<'_> {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_NSSA_STATE_KEY;
    const CF_NAME: &'static str = CF_NSSA_STATE_NAME;

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
            DbError::borsh_cast_message(err, Some("Failed to serialize last state".to_owned()))
        })
    }
}

impl SimpleWritableCell for NSSAStateCellRef<'_> {}

#[derive(Debug)]
pub struct LastFinalizedBlockIdCell(pub Option<u64>);

impl BorshSerialize for LastFinalizedBlockIdCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        Option::<u64>::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for LastFinalizedBlockIdCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Option::<u64>::deserialize_reader(reader).map(LastFinalizedBlockIdCell)
    }
}

impl SimpleStorableCell for LastFinalizedBlockIdCell {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_META_LAST_FINALIZED_BLOCK_ID;
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
                Some("Failed to serialize last finalized block id".to_owned()),
            )
        })
    }
}

impl SimpleReadableCell for LastFinalizedBlockIdCell {}

impl SimpleWritableCell for LastFinalizedBlockIdCell {}

pub struct LatestBlockMetaCellOwned(pub BlockMeta);

impl BorshDeserialize for LatestBlockMetaCellOwned {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        BlockMeta::deserialize_reader(reader).map(LatestBlockMetaCellOwned)
    }
}

impl SimpleStorableCell for LatestBlockMetaCellOwned {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_META_LATEST_BLOCK_META_KEY;
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
        borsh::to_vec(&self.0).map_err(|err| {
            DbError::borsh_cast_message(err, Some("Failed to serialize last block meta".to_owned()))
        })
    }
}

impl SimpleReadableCell for LatestBlockMetaCellOwned {}

pub struct LatestBlockMetaCellRef<'blockmeta>(pub &'blockmeta BlockMeta);

impl BorshSerialize for LatestBlockMetaCellRef<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        BlockMeta::serialize(self.0, writer)
    }
}

impl SimpleStorableCell for LatestBlockMetaCellRef<'_> {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_META_LATEST_BLOCK_META_KEY;
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
            DbError::borsh_cast_message(err, Some("Failed to serialize last block meta".to_owned()))
        })
    }
}

impl SimpleWritableCell for LatestBlockMetaCellRef<'_> {}
