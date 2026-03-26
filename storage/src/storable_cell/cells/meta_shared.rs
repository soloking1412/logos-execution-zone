use borsh::{BorshDeserialize, BorshSerialize};
use common::block::Block;

use crate::{
    CF_BLOCK_NAME, CF_META_NAME, DB_META_FIRST_BLOCK_IN_DB_KEY, DB_META_FIRST_BLOCK_SET_KEY,
    DB_META_LAST_BLOCK_IN_DB_KEY, DbResult,
    error::DbError,
    storable_cell::{SimpleReadableCell, SimpleStorableCell, SimpleWritableCell},
};

#[derive(Debug)]
pub struct LastBlockCell(pub u64);

impl BorshSerialize for LastBlockCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        u64::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for LastBlockCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        u64::deserialize_reader(reader).map(LastBlockCell)
    }
}

impl SimpleStorableCell for LastBlockCell {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_META_LAST_BLOCK_IN_DB_KEY;
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
            DbError::borsh_cast_message(err, Some("Failed to serialize last block id".to_owned()))
        })
    }
}

impl SimpleReadableCell for LastBlockCell {}

impl SimpleWritableCell for LastBlockCell {}

#[derive(Debug)]
pub struct FirstBlockSetCell(pub bool);

impl BorshSerialize for FirstBlockSetCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        bool::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for FirstBlockSetCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        bool::deserialize_reader(reader).map(FirstBlockSetCell)
    }
}

impl SimpleStorableCell for FirstBlockSetCell {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_META_FIRST_BLOCK_SET_KEY;
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
                Some("Failed to serialize first block set flag".to_owned()),
            )
        })
    }
}

impl SimpleReadableCell for FirstBlockSetCell {}

impl SimpleWritableCell for FirstBlockSetCell {}

#[derive(Debug)]
pub struct FirstBlockCell(pub u64);

impl BorshSerialize for FirstBlockCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        u64::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for FirstBlockCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        u64::deserialize_reader(reader).map(FirstBlockCell)
    }
}

impl SimpleStorableCell for FirstBlockCell {
    type KeyParams = ();

    const CELL_NAME: &'static str = DB_META_FIRST_BLOCK_IN_DB_KEY;
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
            DbError::borsh_cast_message(err, Some("Failed to serialize first block id".to_owned()))
        })
    }
}

impl SimpleReadableCell for FirstBlockCell {}

#[derive(Debug)]
pub struct BlockCell(pub Block);

impl BorshSerialize for BlockCell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        Block::serialize(&self.0, writer)
    }
}

impl BorshDeserialize for BlockCell {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Block::deserialize_reader(reader).map(BlockCell)
    }
}

impl SimpleStorableCell for BlockCell {
    type KeyParams = u64;

    const CELL_NAME: &'static str = "block";
    const CF_NAME: &'static str = CF_BLOCK_NAME;

    fn key_constructor(params: Self::KeyParams) -> DbResult<Vec<u8>> {
        // ToDo: Replace with increasing ordering serialization
        borsh::to_vec(&params).map_err(|err| {
            DbError::borsh_cast_message(
                err,
                Some(format!("Failed to serialize {:?}", Self::CELL_NAME)),
            )
        })
    }

    fn value_constructor(&self) -> DbResult<Vec<u8>> {
        borsh::to_vec(&self).map_err(|err| {
            DbError::borsh_cast_message(err, Some("Failed to serialize block".to_owned()))
        })
    }
}

impl SimpleReadableCell for BlockCell {}
