
use serde::{ Deserialize, Serialize };



pub trait Encoder<'a> {
    type In;
    type Encoded: AsRef<[u8]>;
    type Out;

    fn encode(d: Self::In) -> Self::Encoded;
    fn decode(bytes: &'a [u8]) -> Option<Self::Out>;
}

pub struct IVecWrapper<E>
where
    for<'a> E: Encoder<'a>,
{
    ivec: sled::IVec,
    _e: std::marker::PhantomData<E>,
}


impl<E> IVecWrapper<E>
where
    for<'a> E: Encoder<'a>,
{
    pub fn new(ivec: sled::IVec) -> Self {
        Self {
            ivec,
            _e: std::marker::PhantomData,
        }
    }
    pub fn from(value: <E as Encoder>::In) -> Self {
        Self::new(sled::IVec::from(E::encode(value).as_ref()))
    }
    pub fn decode(&self) -> Option<<E as Encoder>::Out> {
        E::decode(&self.ivec)
    }
}

pub struct TreeWrapper<K, V>
where
    for<'a> K: Encoder<'a>,
    for<'a> V: Encoder<'a>,
{
    tree: sled::Tree,
    _k: std::marker::PhantomData<K>,
    _v: std::marker::PhantomData<V>,
}

impl<K, V> TreeWrapper<K, V>
where
    for<'a> K: Encoder<'a>,
    for<'a> V: Encoder<'a>,
{
    pub fn new(tree: sled::Tree) -> Self {
        Self {
            tree,
            _k: std::marker::PhantomData,
            _v: std::marker::PhantomData,
        }
    }

    pub fn insert(
        &self,
        key: String,
        value: <V as Encoder>::In,
    ) -> sled::Result<Option<IVecWrapper<V>>> {
        self.tree
            .insert(key, IVecWrapper::<V>::from(value).ivec)
            .map(|res| res.map(|ivec| IVecWrapper::new(ivec)))
    }

    pub fn get(&self, key: String) -> sled::Result<Option<IVecWrapper<V>>> {
        self.tree
            .get(key)
            .map(|res| res.map(|ivec| IVecWrapper::new(ivec)))
    }
}


pub struct DefaultEncoder();
impl<'a> Encoder<'a> for DefaultEncoder {
    type In = &'a [u8];
    type Encoded = &'a [u8];
    type Out = &'a [u8];

    fn encode(d: Self::In) -> Self::Encoded {
        d
    }

    fn decode(bytes: &'a [u8]) -> Option<Self::Out> {
        Some(bytes)
    }
}


pub struct JSONEncoder<T>(std::marker::PhantomData<T>);
impl<'a, T> Encoder<'a> for JSONEncoder<T>
where
    T: Sized + Serialize + Deserialize<'a>,
    T: 'a,
{
    type In = &'a T;
    type Encoded = Vec<u8>;
    type Out = T;

    fn encode(d: Self::In) -> Self::Encoded {
        serde_json::to_vec(&d).unwrap()
    }

    fn decode(bytes: &'a [u8]) -> Option<Self::Out> {
        serde_json::from_slice(bytes).unwrap()
    }
}