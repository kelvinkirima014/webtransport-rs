use bytes::Bytes;
use webtransport_proto::VarInt;

/// an HTTP/3 Datagram
/// See: <https://www.rfc-editor.org/rfc/rfc9297#section-2.1>
#[derive(Debug)]
pub struct Datagram {
    #[allow(dead_code)]
    q_stream_id: VarInt,
    payload: Bytes,
}

impl Datagram {
    ///Creates a new [`Datagram`] with a given payload
    pub fn new(q_stream_id: VarInt, payload: Bytes) -> Self {
        Datagram {
            q_stream_id,
            payload,
        }
    }

    /// Returns the datagram payload
    pub fn payload(&self) -> &Bytes {
        &self.payload
    }
}
