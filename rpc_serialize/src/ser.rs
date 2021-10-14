use async_trait::async_trait;

#[async_trait]
pub trait AsyncSerialize {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer;
}

#[async_trait]
pub trait AsyncSerializer: Sized {
    type Ok;
    type Error;

    type AsyncSerializeSeq: AsyncSerializeSeq<Ok = Self::Ok, Error = Self::Error> + Send + Sync;
    type AsyncSerializeTuple: AsyncSerializeTuple<Ok = Self::Ok, Error = Self::Error> + Send + Sync;
    type AsyncSerializeTupleStruct: AsyncSerializeTupleStruct<Ok = Self::Ok, Error = Self::Error> + Send + Sync;
    type AsyncSerializeTupleVariant: AsyncSerializeTupleVariant<Ok = Self::Ok, Error = Self::Error> + Send + Sync;
    type AsyncSerializeMap: AsyncSerializeMap<Ok = Self::Ok, Error = Self::Error> + Send + Sync;
    type AsyncSerializeStruct: AsyncSerializeStruct<Ok = Self::Ok, Error = Self::Error> + Send + Sync;
    type AsyncSerializeStructVariant: AsyncSerializeStructVariant<Ok = Self::Ok, Error = Self::Error> + Send + Sync;

    async fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    async fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
    async fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
    async fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
    async fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
    async fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
    async fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
    async fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
    async fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
    async fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
    async fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
    async fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
    async fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
    async fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    async fn serialize_none(self) -> Result<Self::Ok, Self::Error>;
    async fn serialize_some<T: ?Sized + Sync + Send>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: AsyncSerialize;
    async fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;
    async fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>;
    async fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error>;
    async fn serialize_newtype_struct<T: ?Sized + Sync + Send>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: AsyncSerialize;
    async fn serialize_newtype_variant<T: ?Sized + Sync + Send>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: AsyncSerialize;
    async fn serialize_seq(self, len: Option<usize>) -> Result<Self::AsyncSerializeSeq, Self::Error>;
    async fn serialize_tuple(self, len: usize) -> Result<Self::AsyncSerializeTuple, Self::Error>;
    async fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::AsyncSerializeTupleStruct, Self::Error>;
    async fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::AsyncSerializeTupleVariant, Self::Error>;
    async fn serialize_map(self, len: Option<usize>) -> Result<Self::AsyncSerializeMap, Self::Error>;
    async fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::AsyncSerializeStruct, Self::Error>;
    async fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::AsyncSerializeStructVariant, Self::Error>;
}

#[async_trait]
pub trait AsyncSerializeSeq {
    type Ok;
    type Error;

    async fn serialize_element<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: AsyncSerialize;
    async fn end(self) -> Result<Self::Ok, Self::Error>;
}

#[async_trait]
pub trait AsyncSerializeTuple {
    type Ok;
    type Error;

    async fn serialize_element<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: AsyncSerialize;
    async fn end(self) -> Result<Self::Ok, Self::Error>;
}

#[async_trait]
pub trait AsyncSerializeTupleStruct {
    type Ok;
    type Error;

    async fn serialize_field<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: AsyncSerialize;
    async fn end(self) -> Result<Self::Ok, Self::Error>;
}

#[async_trait]
pub trait AsyncSerializeTupleVariant {
    type Ok;
    type Error;

    async fn serialize_field<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: AsyncSerialize;
    async fn end(self) -> Result<Self::Ok, Self::Error>;
}

#[async_trait]
pub trait AsyncSerializeMap {
    type Ok;
    type Error;

    async fn serialize_key<T: ?Sized + Send + Sync>(&mut self, key: &T) -> Result<(), Self::Error> where T: AsyncSerialize;
    async fn serialize_value<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: AsyncSerialize;
    async fn serialize_entry<K: ?Sized + Sync + Send, V: ?Sized + Sync + Send>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
        where K: AsyncSerialize, V: AsyncSerialize
    {
        self.serialize_key(key).await?;
        self.serialize_value(value).await?;
        Ok(())
    }
    async fn end(self) -> Result<Self::Ok, Self::Error>;
}

#[async_trait]
pub trait AsyncSerializeStruct {
    type Ok;
    type Error;

    async fn serialize_field<T: ?Sized + Send + Sync>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: AsyncSerialize;
    async fn end(self) -> Result<Self::Ok, Self::Error>;
}

#[async_trait]
pub trait AsyncSerializeStructVariant {
    type Ok;
    type Error;

    async fn serialize_field<T: ?Sized + Send + Sync>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: AsyncSerialize;
    async fn end(self) -> Result<Self::Ok, Self::Error>;
}

#[async_trait]
impl AsyncSerialize for bool {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_bool(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for isize {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_i64(*self as i64).await
    }
}

#[async_trait]
impl AsyncSerialize for i8 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_i8(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for i16 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_i16(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for i32 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_i32(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for i64 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_i64(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for usize {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_u64(*self as u64).await
    }
}


#[async_trait]
impl AsyncSerialize for u8 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_u8(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for u16 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_u16(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for u32 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_u32(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for u64 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_u64(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for f32 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_f32(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for f64 {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_f64(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for char {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_char(*self).await
    }
}

#[async_trait]
impl AsyncSerialize for str {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        serializer.serialize_str(self).await
    }
}

#[async_trait]
impl<T: AsyncSerialize + Sync + Send> AsyncSerialize for Option<T> {
    async fn serialize<S: Send + Sync>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: AsyncSerializer {
        match &*self {
            Some(v) => serializer.serialize_some(v).await,
            None => serializer.serialize_none().await
        }
    }
}