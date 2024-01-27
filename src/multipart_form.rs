use crate::AnyStdResult;
use bytes::{BufMut, Bytes, BytesMut};
use std::fmt::Write;

pub struct FileMultipartData<'a> {
    pub name: String,
    pub content_type: String,
    pub data: &'a [u8],
}

pub(crate) fn generate_multipart_boundary() -> String {
    format!(
        "----WebKitFormBoundarySlackMorphismRust{}",
        chrono::Utc::now().timestamp()
    )
}

pub(crate) fn create_multipart_file_content<'p, PT, TS>(
    fields: &'p PT,
    multipart_boundary: &str,
    file: Option<FileMultipartData<'p>>,
) -> AnyStdResult<Bytes>
where
    PT: std::iter::IntoIterator<Item = (&'p str, Option<TS>)> + Clone,
    TS: AsRef<str> + 'p + Send,
{
    let capacity = file.as_ref().map(|x| x.data.len()).unwrap_or(0) + 1024;
    let mut output = BytesMut::with_capacity(capacity);
    output.write_str("\r\n")?;

    if let Some(file_to_upload) = file {
        output.write_str("\r\n")?;
        output.write_str("--")?;
        output.write_str(multipart_boundary)?;
        output.write_str("\r\n")?;
        output.write_str(&format!(
            "Content-Disposition: form-data; name=\"file\"; filename=\"{}\"",
            file_to_upload.name
        ))?;
        output.write_str("\r\n")?;
        output.write_str(&format!("Content-Type: {}", file_to_upload.content_type))?;
        output.write_str("\r\n")?;
        output.write_str(&format!("Content-Length: {}", file_to_upload.data.len()))?;
        output.write_str("\r\n")?;
        output.write_str("\r\n")?;
        output.put_slice(file_to_upload.data);
    }

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
