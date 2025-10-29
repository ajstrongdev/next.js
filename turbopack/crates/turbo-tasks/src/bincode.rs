use bincode::{
    BorrowDecode, Decode, Encode,
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
};

pub mod indexmap {
    use std::hash::{BuildHasher, Hash};

    use ::indexmap::IndexMap;

    use super::*;

    pub fn encode<E, K, V, S>(map: &IndexMap<K, V, S>, encoder: &mut E) -> Result<(), EncodeError>
    where
        E: Encoder,
        K: Encode,
        V: Encode,
    {
        usize::encode(&map.len(), encoder)?;
        for (k, v) in map {
            K::encode(k, encoder)?;
            V::encode(v, encoder)?;
        }
        Ok(())
    }

    pub fn decode<Context, D, K, V, S>(decoder: &mut D) -> Result<IndexMap<K, V, S>, DecodeError>
    where
        D: Decoder<Context = Context>,
        K: Decode<Context> + Eq + Hash,
        V: Decode<Context>,
        S: BuildHasher + Default,
    {
        let len = usize::decode(decoder)?;
        let mut map = IndexMap::with_capacity_and_hasher(len, Default::default());
        for _i in 0..len {
            map.insert(K::decode(decoder)?, V::decode(decoder)?);
        }
        Ok(map)
    }

    pub fn borrow_decode<'de, Context, D, K, V, S>(
        decoder: &mut D,
    ) -> Result<IndexMap<K, V, S>, DecodeError>
    where
        D: BorrowDecoder<'de, Context = Context>,
        K: BorrowDecode<'de, Context> + Eq + Hash,
        V: BorrowDecode<'de, Context>,
        S: BuildHasher + Default,
    {
        let len = usize::decode(decoder)?;
        let mut map = IndexMap::with_capacity_and_hasher(len, Default::default());
        for _i in 0..len {
            map.insert(K::borrow_decode(decoder)?, V::borrow_decode(decoder)?);
        }
        Ok(map)
    }
}

pub mod indexset {
    use std::hash::{BuildHasher, Hash};

    use ::indexmap::IndexSet;

    use super::*;

    pub fn encode<E, T, S>(set: &IndexSet<T, S>, encoder: &mut E) -> Result<(), EncodeError>
    where
        E: Encoder,
        T: Encode,
    {
        usize::encode(&set.len(), encoder)?;
        for item in set {
            T::encode(item, encoder)?;
        }
        Ok(())
    }

    pub fn decode<Context, D, T, S>(decoder: &mut D) -> Result<IndexSet<T, S>, DecodeError>
    where
        D: Decoder<Context = Context>,
        T: Decode<Context> + Eq + Hash,
        S: BuildHasher + Default,
    {
        let len = usize::decode(decoder)?;
        let mut set = IndexSet::with_capacity_and_hasher(len, Default::default());
        for _i in 0..len {
            set.insert(T::decode(decoder)?);
        }
        Ok(set)
    }

    pub fn borrow_decode<'de, Context, D, T, S>(
        decoder: &mut D,
    ) -> Result<IndexSet<T, S>, DecodeError>
    where
        D: BorrowDecoder<'de, Context = Context>,
        T: BorrowDecode<'de, Context> + Eq + Hash,
        S: BuildHasher + Default,
    {
        let len = usize::decode(decoder)?;
        let mut set = IndexSet::with_capacity_and_hasher(len, Default::default());
        for _i in 0..len {
            set.insert(T::borrow_decode(decoder)?);
        }
        Ok(set)
    }
}
