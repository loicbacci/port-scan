/// Header of a DNS message.
#[derive(Debug)]
pub struct Header {
    /// (16-bit) Identifier of the message (to help match replies)
    id: u16,
    /// (16-bit) Flags for the header
    flags: HeaderFlags,
    /// (16-bit) Number of entries in question section
    qdcount: u16,
    /// (16-bit) Number of records in answer section
    ancount: u16,
    /// (16-bit) Number of server records in authority records section
    nscount: u16,
    /// (16-bit) Number of records in additional records section
    arcount: u16,
}

impl Header {
    pub fn new(
        id: u16,
        flags: HeaderFlags,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
    ) -> Header {
        Header {
            id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }

    pub fn write_to_buf(&self, buf: &mut Vec<u8>) {
        let flags = &self.flags.to_u16();

        buf.extend_from_slice(&self.id.to_be_bytes());
        buf.extend_from_slice(&flags.to_be_bytes());
        buf.extend_from_slice(&self.qdcount.to_be_bytes());
        buf.extend_from_slice(&self.ancount.to_be_bytes());
        buf.extend_from_slice(&self.nscount.to_be_bytes());
        buf.extend_from_slice(&self.arcount.to_be_bytes());
    }
}

/// List of flags for the DNS message header.
/// Fits into 16 bits
#[derive(Debug)]
pub struct HeaderFlags {
    /// (1-bit) Query or reponse
    qr: QR,
    /// (4-bit) Type of message
    opcode: OPCODE,
    /// (1-bit) Is the server an authority
    /// (set in response)
    aa: bool,
    // (1-bit) Was the message truncated
    tc: bool,
    /// (1-bit) Tell server to pursue the query recursively
    /// (set in query)
    rd: bool,
    /// (1-bit) Is recursive query supported in server
    /// (set in response)
    ra: bool,
    // (3-bits) all zeros
    z: u8,
    /// (4-bit) response code
    rcode: RCODE,
}

impl HeaderFlags {
    pub fn new(
        qr: QR,
        opcode: OPCODE,
        aa: bool,
        tc: bool,
        rd: bool,
        ra: bool,
        rcode: RCODE,
    ) -> HeaderFlags {
        HeaderFlags {
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z: 0,
            rcode,
        }
    }

    fn to_u16(&self) -> u16 {
        let mut res: u16 = 0;

        // Add values
        res |= (self.qr as u16) << 15;
        res |= (self.opcode as u16) << 14;
        res |= (self.aa as u16) << 10;
        res |= (self.tc as u16) << 9;
        res |= (self.rd as u16) << 8;
        res |= (self.ra as u16) << 7;
        res |= (self.z as u16) << 4;
        res |= self.rcode as u16;

        res
    }
}

// ============ ENUMS ================
#[derive(Copy, Clone, Debug)]
pub enum QR {
    Query = 0,
    Response = 1,
}

#[derive(Copy, Clone, Debug)]
pub enum OPCODE {
    Query = 0,
    InverseQuery = 1,
    Status = 2,
}

#[derive(Copy, Clone, Debug)]
pub enum RCODE {
    Success = 0,
    /// Was unable to interpret query
    FormatError = 1,
    /// Unable to process due to a problem with the server
    ServerFailure = 2,
    /// Domain name does not exist
    NameError = 3,
    /// The query is not supported
    NotImplemented = 4,
    /// Refused to perform the operation
    Refused = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_flags_to_u16() {
        let header_flags = HeaderFlags::new(
            QR::Response,
            OPCODE::Query,
            true,
            false,
            true,
            false,
            RCODE::NotImplemented,
        );

        let expected = 0b1000010100000100u16;

        assert_eq!(header_flags.to_u16(), expected);
    }
}
