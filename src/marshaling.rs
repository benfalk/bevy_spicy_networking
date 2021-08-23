use crate::NetworkPacket;

type Result<T> = std::result::Result<T, String>;

/// This is the trait you must implement if you
/// plan on using your own encode/decode strategy
pub trait PacketMarshalingStrategy {
    /// Given a network packet, how many bytes will it take
    /// to serialize.  This must be exact as the serialize_into
    /// method will be given a buffer of exactly that length to use
    fn serialized_size(packet: &NetworkPacket) -> Result<usize>;

    /// Given a buffer and packet, this is the strategy you need
    /// to serialze a packet and fill it into the buffer provided
    fn serialize_into(buffer: &mut[u8], packet: &NetworkPacket) -> Result<()>;

    /// Given a buffer where every byte in it was determined to be
    /// the packet, this must decode the bytes and return a network
    /// packet to be dispatched
    fn deserialize(buffer: &[u8]) -> Result<NetworkPacket>;
}

/// This is the default encoding strategy and is essentially
/// a wrapper around the `bincode` crate.  It is what allows
/// you to have out of the box encoding with serde if you
/// choose.
#[derive(Debug, Copy, Clone)]
pub struct Bincode;

impl PacketMarshalingStrategy for Bincode {
    fn serialized_size(packet: &NetworkPacket) -> Result<usize> {
        match bincode::serialized_size(&packet) {
            Ok(size) => Ok(size as usize),
            Err(err) => Err(format!("{}", err))
        }
    }

    fn serialize_into(buffer: &mut[u8], packet: &NetworkPacket) -> Result<()> {
        match bincode::serialize_into(&mut buffer[..], &packet) {
            Ok(()) => Ok(()),
            Err(err) => Err(format!("{}", err))
        }
    }

    fn deserialize(buffer: &[u8]) -> Result<NetworkPacket> {
        match bincode::deserialize(&buffer[..]) {
            Ok(packet) => Ok(packet),
            Err(err) => Err(format!("{}", err))
        }
    }
}
