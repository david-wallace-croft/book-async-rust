use ::book_async_rust::ch10::data::Data;
use ::std::io::Cursor;
use ::std::io::Error;

#[test]
fn test_serialize_deserialize() -> Result<(), Error> {
  let data: Data = Data {
    field1: 1,
    field2: 2,
    field3: "3".into(),
  };

  let serialization_result: Result<Vec<u8>, Error> = data.serialize();

  let serialized: Vec<u8> = serialization_result?;

  let inner: &[u8] = &serialized[..];

  let mut cursor: Cursor<&[u8]> = Cursor::new(inner);

  let deserialization_result: Result<Data, Error> =
    Data::deserialize(&mut cursor);

  let deserialized_data: Data = deserialization_result?;

  assert_eq!(data, deserialized_data);

  Ok(())
}
