use clap::Parser;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug, Parser, PartialEq, Clone)]
#[clap(author, version, about, long_about= None)]
pub(crate) struct Args {
    #[clap(
        short = 'd',
        long,
        help = "Path to your delta root dir, default to current directory"
    )]
    delta_path: Option<String>,

    #[clap(
        short = 'm',
        long,
        help = "The memory pool limitation (e.g. '10g'), default to None (no limit)",
        validator(is_valid_memory_pool_size)
    )]
    memory_limit: Option<String>,

    #[clap(
        long,
        help = "Specify the memory pool type 'greedy' or 'fair', default to 'greedy'"
    )]
    mem_pool_type: Option<PoolType>,
}

#[derive(Debug)]
pub struct JDBCConfig {
    pub(crate) delta_path: String,
    pub(crate) memory_limit: Option<String>,
    pub(crate) mem_pool_type: Option<PoolType>,
}

impl From<Args> for JDBCConfig {
    fn from(args: Args) -> Self {
        JDBCConfig {
            delta_path: args.delta_path.unwrap_or_else(|| ".".to_string()),
            memory_limit: args.memory_limit,
            mem_pool_type: args.mem_pool_type,
        }
    }
}

fn is_valid_memory_pool_size(size: &str) -> Result<(), String> {
    match extract_memory_pool_size(size) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[derive(Debug, Clone, Copy)]
enum ByteUnit {
    Byte,
    KiB,
    MiB,
    GiB,
    TiB,
}

impl ByteUnit {
    fn multiplier(&self) -> usize {
        match self {
            ByteUnit::Byte => 1,
            ByteUnit::KiB => 1 << 10,
            ByteUnit::MiB => 1 << 20,
            ByteUnit::GiB => 1 << 30,
            ByteUnit::TiB => 1 << 40,
        }
    }
}

pub fn extract_memory_pool_size(size: &str) -> Result<usize, String> {
    fn byte_suffixes() -> &'static HashMap<&'static str, ByteUnit> {
        static BYTE_SUFFIXES: OnceLock<HashMap<&'static str, ByteUnit>> = OnceLock::new();
        BYTE_SUFFIXES.get_or_init(|| {
            let mut m = HashMap::new();
            m.insert("b", ByteUnit::Byte);
            m.insert("k", ByteUnit::KiB);
            m.insert("kb", ByteUnit::KiB);
            m.insert("m", ByteUnit::MiB);
            m.insert("mb", ByteUnit::MiB);
            m.insert("g", ByteUnit::GiB);
            m.insert("gb", ByteUnit::GiB);
            m.insert("t", ByteUnit::TiB);
            m.insert("tb", ByteUnit::TiB);
            m
        })
    }

    fn suffix_re() -> &'static regex::Regex {
        static SUFFIX_REGEX: OnceLock<regex::Regex> = OnceLock::new();
        SUFFIX_REGEX.get_or_init(|| regex::Regex::new(r"^(-?[0-9]+)([a-z]+)?$").unwrap())
    }

    let lower = size.to_lowercase();
    if let Some(caps) = suffix_re().captures(&lower) {
        let num_str = caps.get(1).unwrap().as_str();
        let num = num_str
            .parse::<usize>()
            .map_err(|_| format!("Invalid numeric value in memory pool size '{}'", size))?;

        let suffix = caps.get(2).map(|m| m.as_str()).unwrap_or("b");
        let unit = byte_suffixes()
            .get(suffix)
            .ok_or_else(|| format!("Invalid memory pool size '{}'", size))?;

        Ok(num * unit.multiplier())
    } else {
        Err(format!("Invalid memory pool size '{}'", size))
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum PoolType {
    Greedy,
    Fair,
}

impl FromStr for PoolType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Greedy" | "greedy" => Ok(PoolType::Greedy),
            "Fair" | "fair" => Ok(PoolType::Fair),
            _ => Err(format!("Invalid memory pool type '{}'", s)),
        }
    }
}
