pub use rpc_serialize_derive;

pub mod ser;
pub mod binary;

#[cfg(test)]
mod tests {
    mod serializing {
        use crate::binary::BinarySerializer;
        use crate::rpc_serialize_derive::AsyncSerialize;

        #[derive(AsyncSerialize)]
        pub struct Test {
            num: i64,
            num2: i64,
        }

        #[tokio::test]
        async fn test_i32_serializing() {
            use crate::ser::AsyncSerialize;

            let expected: i32 = 10;
            let mut output = <Vec<u8>>::new();
            let mut serializer = BinarySerializer { output: &mut output };

            expected.serialize(&mut serializer).await.unwrap();

            assert_eq!(expected.to_ne_bytes(), output.as_slice());
        }

        #[tokio::test]
        async fn test_i64_serializing() {
            use crate::ser::AsyncSerialize;

            let expected: i64 = 10;
            let mut output = <Vec<u8>>::new();
            let mut serializer = BinarySerializer { output: &mut output };

            expected.serialize(&mut serializer).await.unwrap();

            assert_eq!(expected.to_ne_bytes(), output.as_slice());
        }

        #[tokio::test]
        async fn test_struct_serializing() {
            use crate::ser::AsyncSerialize;

            let expected = Test { num: 16, num2: 32 };
            let mut output = <Vec<u8>>::new();
            let mut serializer = BinarySerializer { output: &mut output };

            expected.serialize(&mut serializer).await.unwrap();

            assert_eq!([ 16i64.to_ne_bytes(), 32i64.to_ne_bytes() ].concat(), output.as_slice());
        }
    }
}
