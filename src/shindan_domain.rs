use anyhow::anyhow;
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

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "JP" => Ok(Self::Jp),
            "EN" => Ok(Self::En),
            "CN" => Ok(Self::Cn),
            "KR" => Ok(Self::Kr),
            "TH" => Ok(Self::Th),
            _ => Err(anyhow!("Invalid domain")),
        }
    }
}

impl TryFrom<&str> for ShindanDomain {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for ShindanDomain {
    type Error = anyhow::Error;

    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        value.parse()
    }
}
