use std::fmt;

#[derive(Debug)]
pub struct Question {
    entries: Vec<QuestionEntry>,
}

impl Question {
    pub fn new(entries: Vec<(&str, QTYPE, QCLASS)>) -> Self {
        let q_entries = entries
            .into_iter()
            .map(|(dn, qt, qc)| QuestionEntry::new(dn, qt, qc))
            .collect();

        Question { entries: q_entries }
    }

    pub fn write_to_buf(&self, buf: &mut Vec<u8>) {
        for entry in &self.entries {
            entry.write_to_buf(buf);
        }
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, entry) in self.entries.iter().enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }
            write!(f, "{entry}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct QuestionEntry {
    /// The domain name
    qname: QNAME,
    /// (16-bit) The type of the query
    qtype: QTYPE,
    /// (16-bit) The class of the query
    qclass: QCLASS,
}

impl QuestionEntry {
    fn new(domain_name: &str, qtype: QTYPE, qclass: QCLASS) -> QuestionEntry {
        let qname = QNAME::new(domain_name);

        QuestionEntry {
            qname,
            qtype,
            qclass,
        }
    }

    pub fn write_to_buf(&self, buf: &mut Vec<u8>) {
        self.qname.write_to_buf(buf);

        let qtype: u16 = self.qtype as u16;
        let qclass: u16 = self.qclass as u16;

        buf.extend_from_slice(&qtype.to_be_bytes());
        buf.extend_from_slice(&qclass.to_be_bytes());
    }
}

impl fmt::Display for QuestionEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?} {:?}", self.qname, self.qtype, self.qclass)
    }
}

#[derive(Debug)]
struct QNAME {
    /// Each label of the domain name
    labels: Vec<Label>,
}

impl QNAME {
    /// Creates a question QNAME from a domain name
    fn new(domain_name: &str) -> QNAME {
        // Split the domain name into labels
        let label_strs = domain_name.split('.');
        let labels = label_strs.map(|l| Label::new(&l)).collect();

        QNAME { labels }
    }

    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        for label in &self.labels {
            label.write_to_buf(buf);
        }

        // Write termination
        buf.push(0);
    }
}

impl fmt::Display for QNAME {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, label) in self.labels.iter().enumerate() {
            if i != 0 {
                f.write_str(".")?;
            }

            label.fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Label {
    /// The letters of the label
    letters: Vec<u8>,
}

impl Label {
    fn new(label_str: &str) -> Self {
        Self::from_bytes(label_str.as_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        Label {
            letters: bytes.to_vec(),
        }
    }

    fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push(self.letters.len() as u8);
        buf.extend_from_slice(self.letters.as_slice());
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.letters.escape_ascii())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum QTYPE {
    /// Host address
    A = 1,
    /// Authoritative name server
    NS = 2,
    /// (Obsolete) Mail destination
    MD = 3,
    /// (Obsolete) Mail forwareder
    MF = 4,
    /// Cononical name for an alias
    CNAME = 5,
    /// Start of zone of authority
    SOA = 6,
    /// (Experimental) Mailbox domain name
    MB = 7,
    /// (Experimental) Mail group member
    MG = 8,
    /// (Experimental) Mail rename domain name
    MR = 9,
    // (Experimental) Null record
    NULL = 10,
    /// Well known service description
    WKS = 11,
    /// Domain name pointer
    PTR = 12,
    /// Host information
    HINFO = 13,
    /// Mailbox or mail list information
    MINFO = 14,
    /// Mail exchange
    MX = 15,
    /// Text strings
    TXT = 16,

    /// Request transfer of an entire zone
    AXFR = 252,
    /// Request for mailbox-related records
    MAILB = 253,
    /// (Obsolete) Request for mail agent records
    MAILA = 254,
    /// Request all records
    STAR = 255,
}

#[derive(Copy, Clone, Debug)]
pub enum QCLASS {
    /// The internet
    IN = 1,
    /// (Obsolete) CSNET
    CS = 2,
    /// CHAOS
    CH = 3,
    /// Hesiod [Dyer 87]
    HS = 4,

    /// Any class
    STAR = 255,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qname_write_to_buf() {
        let dn = "test.abc";
        let qname = QNAME::new(dn);

        let exp = b"\x04test\x03abc\x00";
        let mut buf: Vec<u8> = vec![];
        qname.write_to_buf(&mut buf);

        assert_eq!(exp.as_slice(), buf.as_slice());
    }

    #[test]
    fn test_qname_display() {
        let dn = "test.abc";
        let qname = QNAME::new(dn);
        let qname_str = qname.to_string();

        assert_eq!(dn.to_string(), qname_str);
    }

    #[test]
    fn test_label_write_to_buf() {
        let dn = "test";
        let label = Label::new(dn);

        let exp = b"\x04test";
        let mut buf: Vec<u8> = vec![];
        label.write_to_buf(&mut buf);

        assert_eq!(exp.as_slice(), buf.as_slice());
    }

    #[test]
    fn test_label_display() {
        let dn = "test";
        let label = Label::new(dn);
        let label_str = label.to_string();

        assert_eq!(dn.to_string(), label_str);
    }

    #[test]
    fn test_label_display_arbitrary_bytes() {
        let dn = b"\x01\x02\x03\x04";
        let label = Label {
            letters: dn.to_vec(),
        };
        let label_str = label.to_string();

        assert_eq!("\\x01\\x02\\x03\\x04", label_str);
    }
}
