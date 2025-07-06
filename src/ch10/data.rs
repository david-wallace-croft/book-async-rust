use ::std::io::{self, Cursor, Read, Write};

#[derive(Debug, Eq, PartialEq)]
pub struct Data {
  pub field1: u32,
  pub field2: u16,
  pub field3: String,
}

impl Data {
  pub fn deserialize(cursor: &mut Cursor<&[u8]>) -> io::Result<Data> {
    let mut field1_bytes: [u8; 4] = [0; 4];

    let mut field2_bytes: [u8; 2] = [0; 2];

    cursor.read_exact(&mut field1_bytes)?;

    cursor.read_exact(&mut field2_bytes)?;

    let field1: u32 = u32::from_ne_bytes(field1_bytes);

    let field2: u16 = u16::from_ne_bytes(field2_bytes);

    let mut len_bytes: [u8; 4] = [0; 4];

    cursor.read_exact(&mut len_bytes)?;

    let len: usize = u32::from_ne_bytes(len_bytes) as usize;

    let mut field3_bytes: Vec<u8> = vec![0u8; len];

    cursor.read_exact(&mut field3_bytes)?;

    let field3: String = String::from_utf8(field3_bytes).map_err(|_| {
      io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")
    })?;

    let data: Data = Data {
      field1,
      field2,
      field3,
    };

    Ok(data)
  }

  pub fn serialize(&self) -> io::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();

    let _: usize = bytes.write(&self.field1.to_ne_bytes())?;

    let _: usize = bytes.write(&self.field2.to_ne_bytes())?;

    let field3_len: u32 = self.field3.len() as u32;

    let _: usize = bytes.write(&field3_len.to_ne_bytes())?;

    bytes.extend_from_slice(self.field3.as_bytes());

    Ok(bytes)
  }
}
