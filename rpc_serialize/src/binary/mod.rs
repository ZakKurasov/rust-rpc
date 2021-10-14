use tokio::io::{AsyncWrite};
use async_trait::async_trait;
use crate::ser::AsyncSerialize;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
}

pub struct BinarySerializer<'a> {
    pub output: &'a mut (dyn AsyncWrite + Unpin + Sync + Send),
}

unsafe impl<'a> Send for BinarySerializer<'a> {}

unsafe impl<'a> Sync for BinarySerializer<'a> {}

#[async_trait]
impl<'a, 'b> crate::ser::AsyncSerializer for &'a mut BinarySerializer<'b> {
    type Ok = ();
    type Error = Error;

    type AsyncSerializeSeq = Self;
    type AsyncSerializeTuple = Self;
    type AsyncSerializeTupleStruct = Self;
    type AsyncSerializeTupleVariant = Self;
    type AsyncSerializeMap = Self;
    type AsyncSerializeStruct = Self;
    type AsyncSerializeStructVariant = Self;

    async fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(u8::from(v)).await
    }

    async fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_i64(mut self, v: i64) -> Result<(), Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        use tokio::io::AsyncWriteExt;
        self.output.write(&v.to_ne_bytes()).await.unwrap();
        Ok(())
    }

    async fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(u32::from(v)).await
    }

    async fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes()).await
    }

    async fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use crate::ser::AsyncSerializeSeq;

        let mut seq = self.serialize_seq(Some(v.len())).await.unwrap();
        for i in v {
            seq.serialize_element(i).await?;
        };
        seq.end().await
    }

    async fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    async fn serialize_some<T: ?Sized + Sync + Send>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: crate::ser::AsyncSerialize {
        value.serialize(self).await
    }

    async fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    async fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit().await
    }

    async fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant).await
    }

    async fn serialize_newtype_struct<T: ?Sized + Sync + Send>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: crate::ser::AsyncSerialize {
        value.serialize(self).await
    }

    async fn serialize_newtype_variant<T: ?Sized + Sync + Send>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: crate::ser::AsyncSerialize {
        variant.serialize(&mut *self).await?;
        value.serialize(&mut *self).await
    }

    async fn serialize_seq(self, len: Option<usize>) -> Result<Self::AsyncSerializeSeq, Self::Error> {
        len.serialize(&mut *self).await?;
        Ok(self)
    }

    async fn serialize_tuple(self, len: usize) -> Result<Self::AsyncSerializeTuple, Self::Error> {
        len.serialize(&mut *self).await?;
        Ok(self)
    }

    async fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::AsyncSerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    async fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::AsyncSerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    async fn serialize_map(self, _len: Option<usize>) -> Result<Self::AsyncSerializeMap, Self::Error> {
        Ok(self)
    }

    async fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::AsyncSerializeStruct, Self::Error> {
        Ok(self)
    }

    async fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::AsyncSerializeStructVariant, Self::Error> {
        Ok(self)
    }
}

#[async_trait]
impl<'a, 'b> crate::ser::AsyncSerializeSeq for &'a mut BinarySerializer<'b> {
    type Ok = ();
    type Error = Error;

    async fn serialize_element<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: crate::ser::AsyncSerialize {
        value.serialize(&mut **self).await
    }

    async fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<'a, 'b> crate::ser::AsyncSerializeTuple for &'b mut BinarySerializer<'a> {
    type Ok = ();
    type Error = Error;

    async fn serialize_element<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: crate::ser::AsyncSerialize {
        value.serialize(&mut **self).await
    }

    async fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<'a, 'b> crate::ser::AsyncSerializeTupleStruct for &'b mut BinarySerializer<'a> {
    type Ok = ();
    type Error = Error;

    async fn serialize_field<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: crate::ser::AsyncSerialize {
        value.serialize(&mut **self).await
    }

    async fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<'a, 'b> crate::ser::AsyncSerializeTupleVariant for &'b mut BinarySerializer<'a> {
    type Ok = ();
    type Error = Error;

    async fn serialize_field<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: crate::ser::AsyncSerialize {
        value.serialize(&mut **self).await
    }

    async fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<'a, 'b> crate::ser::AsyncSerializeMap for &'b mut BinarySerializer<'a> {
    type Ok = ();
    type Error = Error;

    async fn serialize_key<T: ?Sized + Send + Sync>(&mut self, key: &T) -> Result<(), Self::Error> where T: crate::ser::AsyncSerialize {
        key.serialize(&mut **self).await
    }

    async fn serialize_value<T: ?Sized + Send + Sync>(&mut self, value: &T) -> Result<(), Self::Error> where T: crate::ser::AsyncSerialize {
        value.serialize(&mut **self).await
    }

    async fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<'a, 'b> crate::ser::AsyncSerializeStruct for &'b mut BinarySerializer<'a> {
    type Ok = ();
    type Error = Error;

    async fn serialize_field<T: ?Sized + Send + Sync>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error> where T: crate::ser::AsyncSerialize {
        value.serialize(&mut **self).await
    }

    async fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[async_trait]
impl<'a, 'b> crate::ser::AsyncSerializeStructVariant for &'b mut BinarySerializer<'a> {
    type Ok = ();
    type Error = Error;

    async fn serialize_field<T: ?Sized + Send + Sync>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: crate::ser::AsyncSerialize {
        use crate::ser::AsyncSerialize;

        key.serialize(&mut **self).await?;
        value.serialize(&mut **self).await
    }

    async fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}