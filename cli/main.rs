use std::fmt;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use multibase::Base;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    /// The mode
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(StructOpt, Debug)]
enum Mode {
    #[structopt(name = "encode")]
    Encode {
        /// The base to use for encoding.
        #[structopt(short = "b", long = "base", default_value = "base58btc")]
        base: StrBase,
        /// The data need to be encoded.
        #[structopt(short = "i", long = "input")]
        input: String,
    },
    #[structopt(name = "decode")]
    Decode {
        /// The data need to be decoded.
        #[structopt(short = "i", long = "input")]
        input: String,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::from_args();
    match opts.mode {
        Mode::Encode { base, input } => encode(base, input.as_bytes()),
        Mode::Decode { input } => decode(&input),
    }
}

#[derive(Debug)]
struct StrBase(Base);

impl fmt::Display for StrBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_str = match self.0 {
            Base::Identity => "identity",
            Base::Base2 => "base2",
            Base::Base8 => "base8",
            Base::Base10 => "base10",
            Base::Base16Lower => "base16",
            Base::Base16Upper => "base16upper",
            Base::Base32HexLower => "base32hex",
            Base::Base32HexUpper => "base32hexupper",
            Base::Base32HexPadLower => "base32hexpad",
            Base::Base32HexPadUpper => "base32hexpadupper",
            Base::Base32Lower => "base32",
            Base::Base32Upper => "base32upper",
            Base::Base32PadLower => "base32pad",
            Base::Base32PadUpper => "base32padupper",
            Base::Base32Z => "base32z",
            Base::Base36Lower => "base36lower",
            Base::Base36Upper => "base36upper",
            Base::Base58Flickr => "base58flickr",
            Base::Base58Btc => "base58btc",
            Base::Base64 => "base64",
            Base::Base64Pad => "base64pad",
            Base::Base64Url => "base64url",
            Base::Base64UrlPad => "base64urlpad",
            Base::Base256Emoji => "base256emoji",
        };
        write!(f, "{}", base_str)
    }
}

impl FromStr for StrBase {
    type Err = Error;

    fn from_str(base_str: &str) -> Result<Self, Self::Err> {
        let base = match base_str {
            "identity" => Ok(Base::Identity),
            "base2" => Ok(Base::Base2),
            "base8" => Ok(Base::Base8),
            "base10" => Ok(Base::Base10),
            "base16" => Ok(Base::Base16Lower),
            "base16upper" => Ok(Base::Base16Upper),
            "base32hex" => Ok(Base::Base32HexLower),
            "base32hexupper" => Ok(Base::Base32HexUpper),
            "base32hexpad" => Ok(Base::Base32HexPadLower),
            "base32hexpadupper" => Ok(Base::Base32HexPadUpper),
            "base32" => Ok(Base::Base32Lower),
            "base32upper" => Ok(Base::Base32Upper),
            "base32pad" => Ok(Base::Base32PadLower),
            "base32padupper" => Ok(Base::Base32PadUpper),
            "base32z" => Ok(Base::Base32Z),
            "base36lower" => Ok(Base::Base36Lower),
            "base36upper" => Ok(Base::Base36Upper),
            "base58flickr" => Ok(Base::Base58Flickr),
            "base58btc" => Ok(Base::Base58Btc),
            "base64" => Ok(Base::Base64),
            "base64pad" => Ok(Base::Base64Pad),
            "base64url" => Ok(Base::Base64Url),
            "base64urlpad" => Ok(Base::Base64UrlPad),
            "base256emoji" => Ok(Base::Base256Emoji),
            _ => return Err(anyhow!("Unknown base: {:?}", base_str)),
        };
        base.map(Self)
    }
}

impl From<StrBase> for Base {
    fn from(base: StrBase) -> Self {
        base.0
    }
}

fn encode(base: StrBase, input: &[u8]) -> Result<()> {
    log::debug!("Encode {:?} with {}", input, base);
    let result = multibase::encode(base.into(), input);
    println!("Result: {}", result);
    Ok(())
}

fn decode(input: &str) -> Result<()> {
    log::debug!("Decode {:?}", input);
    let (base, result) = multibase::decode(input)?;
    println!("Result: {}, {:?}", StrBase(base), result);
    Ok(())
}
