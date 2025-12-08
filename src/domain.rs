use anyhow::Result;
use std::fmt;
use std::str::FromStr;

/// A domain of ShindanMaker.
#[derive(Debug, Clone, Copy)]
pub enum ShindanDomain {
    Jp,
    En,
    Cn,
    Kr,
    Th,
}

impl fmt::Display for ShindanDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url = match self {
            Self::Jp => "https://shindanmaker.com/",
            Self::En => "https://en.shindanmaker.com/",
            Self::Cn => "https://cn.shindanmaker.com/",
            Self::Kr => "https://kr.shindanmaker.com/",
            Self::Th => "https://th.shindanmaker.com/",
        };
        write!(f, "{}", url)
    }
}

impl FromStr for ShindanDomain {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "JP" => Ok(Self::Jp),
            "EN" => Ok(Self::En),
            "CN" => Ok(Self::Cn),
            "KR" => Ok(Self::Kr),
            "TH" => Ok(Self::Th),
            _ => Err(anyhow::anyhow!("Invalid domain")),
        }
    }
}
