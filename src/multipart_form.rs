use crate::AnyStdResult;
use bytes::{BufMut, Bytes, BytesMut};
use std::fmt::Write;

pub fn generate_multipart_boundary() -> String {
    format!(
        "----WebKitFormBoundarySlackMorphismRust{}",
        chrono::Utc::now().timestamp()
    )
}

pub struct FileMultipartData<'a> {
    pub name: &'a str,
    pub content_type: &'a str,
    pub data: &'a [u8],
}

pub fn create_multipart_file_content<'p, PT, TS>(
    fields: &'p PT,
    multipart_boundary: &str,
    file: FileMultipartData<'p>,
) -> AnyStdResult<Bytes>
where
    PT: std::iter::IntoIterator<Item = (&'p str, Option<TS>)> + Clone,
    TS: AsRef<str> + 'p + Send,
{
    let mut output = BytesMut::with_capacity(file.data.len() + 512);
    output.write_str("\r\n")?;
    output.write_str("\r\n")?;
    output.write_str("--")?;
    output.write_str(multipart_boundary)?;
    output.write_str("\r\n")?;
    output.write_str(&format!(
        "Content-Disposition: form-data; name=\"file\"; filename=\"{}\"",
        file.name
    ))?;
    output.write_str("\r\n")?;
    output.write_str(&format!("Content-Type: {}", file.content_type))?;
    output.write_str("\r\n")?;
    output.write_str(&format!("Content-Length: {}", file.data.len()))?;
    output.write_str("\r\n")?;
    output.write_str("\r\n")?;
    output.put_slice(file.data);

    for (k, mv) in fields.clone().into_iter() {
        if let Some(v) = mv {
            let vs = v.as_ref();
            output.write_str("\r\n")?;
            output.write_str("--")?;
            output.write_str(multipart_boundary)?;
            output.write_str("\r\n")?;
            output.write_str(&format!("Content-Disposition: form-data; name=\"{}\"", k))?;
            output.write_str("\r\n")?;
            output.write_str(&format!("Content-Length: {}", vs.len()))?;
            output.write_str("\r\n")?;
            output.write_str("\r\n")?;
            output.write_str(vs)?;
        }
    }

    output.write_str("\r\n")?;
    output.write_str("--")?;
    output.write_str(multipart_boundary)?;

    Ok(output.freeze())
}
