use crate::Error;

pub(super) fn read(
    mut response: ureq::http::Response<ureq::Body>,
    endpoint: &str,
) -> Result<(u16, String), Error> {
    let status = response.status().as_u16();
    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|source| Error::Unavailable {
            endpoint: endpoint.to_owned(),
            source,
        })?;
    Ok((status, body))
}
