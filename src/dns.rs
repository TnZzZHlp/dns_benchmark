use bytes::{BufMut, Bytes, BytesMut};
use rand::{distr::Alphanumeric, Rng};
use std::net::SocketAddr;

use crate::cli;

#[derive(Debug, Clone, Copy)]
pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub questions: u16,
    pub answers: u16,
    pub authority: u16,
    pub additional: u16,
}

impl DnsHeader {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            flags: 0x0100, // Standard query, recursion desired
            questions: 1,
            answers: 0,
            authority: 0,
            additional: 0,
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.put_u16(self.id);
        buf.put_u16(self.flags);
        buf.put_u16(self.questions);
        buf.put_u16(self.answers);
        buf.put_u16(self.authority);
        buf.put_u16(self.additional);
        buf.freeze()
    }
}

#[derive(Debug)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl DnsQuestion {
    pub fn new(name: String) -> Self {
        Self {
            name,
            qtype: 1,  // A record
            qclass: 1, // IN class
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        
        for label in self.name.split('.') {
            buf.put_u8(label.len() as u8);
            buf.put_slice(label.as_bytes());
        }
        buf.put_u8(0); // End of name
        
        buf.put_u16(self.qtype);
        buf.put_u16(self.qclass);
        
        buf.freeze()
    }
}

#[derive(Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub question: DnsQuestion,
}

impl DnsPacket {
    pub fn new(domain: String, mode: &cli::TestMode) -> Self {
        let id = rand::rng().random_range(0..65535);
        
        let final_domain = match mode {
            cli::TestMode::SameDomain => domain,
            cli::TestMode::RandomSubdomain => {
                let random_prefix: String = rand::rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect();
                format!("{}.{}", random_prefix, domain)
            }
        };

        Self {
            header: DnsHeader::new(id),
            question: DnsQuestion::new(final_domain),
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.put(self.header.to_bytes());
        buf.put(self.question.to_bytes());
        buf.freeze()
    }
}

#[derive(Clone)]
pub struct DnsClient {
    pub target: SocketAddr,
    pub timeout: std::time::Duration,
}

impl DnsClient {
    pub fn new(target: SocketAddr, timeout: std::time::Duration) -> Self {
        Self { target, timeout }
    }

    pub async fn send_query(&self, packet: &DnsPacket) -> anyhow::Result<()> {
        let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
        let data = packet.to_bytes();
        
        socket.send_to(&data, self.target).await?;
        
        let mut buf = vec![0u8; 512];
        let _ = tokio::time::timeout(self.timeout, socket.recv_from(&mut buf)).await?;
        
        Ok(())
    }
}

pub struct DnsBenchmark {
    pub client: DnsClient,
    pub domain: String,
    pub rate: u32,
    pub mode: cli::TestMode,
}

impl DnsBenchmark {
    pub fn new(
        target: SocketAddr,
        domain: String,
        rate: u32,
        timeout: std::time::Duration,
        mode: cli::TestMode,
    ) -> Self {
        let client = DnsClient::new(target, timeout);
        Self {
            client,
            domain,
            rate,
            mode,
        }
    }
}