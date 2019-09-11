use core::{fmt, str::FromStr};
use exitfailure::ExitFailure;
use failure::{format_err, Error};
use multibase::Base;
use std::io::{self, Read, Write};
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
        #[structopt(short = "b", long = "base", default_value = "base58-btc")]
        base: StrBase,
    },
    #[structopt(name = "decode")]
    Decode,
}

fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    let opts = Opts::from_args();
    match opts.mode {
        Mode::Encode { base } => encode(base),
        Mode::Decode => decode(),
    }
}

#[derive(Debug)]
struct StrBase(Base);

impl fmt::Display for StrBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base_str = match self.0 {
            Base::Base2 => "base2",
            Base::Base8 => "base8",
            Base::Base10 => "base10",
            Base::Base16Upper => "base16",
            Base::Base16Lower => "base16-lower",
            Base::Base32UpperNoPad => "base32",
            Base::Base32UpperPad => "base32-pad",
            Base::Base58flickr => "base58-flickr",
            Base::Base58btc => "base58-btc",
            Base::Base64UpperNoPad => "base64",
            Base::Base64UpperPad => "base64-pad",
            Base::Base64UrlUpperNoPad => "base64-url",
            Base::Base64UrlUpperPad => "base64-url-pad",
        };
        write!(f, "{}", base_str)
    }
}

impl FromStr for StrBase {
    type Err = Error;

    fn from_str(base_str: &str) -> Result<Self, Self::Err> {
        let base = match base_str {
            "base2" => Ok(Base::Base2),
            "base8" => Ok(Base::Base8),
            "base10" => Ok(Base::Base10),
            "base16" => Ok(Base::Base16Upper),
            "base16-lower" => Ok(Base::Base16Lower),
            "base32" => Ok(Base::Base32UpperNoPad),
            "base32-pad" => Ok(Base::Base32UpperPad),
            "base58-flickr" => Ok(Base::Base58flickr),
            "base58-btc" => Ok(Base::Base58btc),
            "base64" => Ok(Base::Base64UpperNoPad),
            "base64-pad" => Ok(Base::Base64UpperPad),
            "base64-url" => Ok(Base::Base64UrlUpperNoPad),
            "base64-url-pad" => Ok(Base::Base64UrlUpperPad),
            _ => Err(format_err!("Unknown base {:?}", base_str)),
        };
        base.map(Self)
    }
}

impl From<StrBase> for Base {
    fn from(base: StrBase) -> Self {
        base.0
    }
}

fn encode(base: StrBase) -> Result<(), ExitFailure> {
    log::debug!("encoding with {}", base);
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer)?;
    log::debug!("read {:?} from stdin", buffer);
    let result = multibase::encode(base.into(), buffer.as_slice());
    stdout.write_all(result.as_bytes())?;
    Ok(())
}

fn decode() -> Result<(), ExitFailure> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    log::debug!("read {:?} from stdin", buffer);
    let (base, result) = multibase::decode(buffer.as_str())?;
    log::debug!("detected {}", StrBase(base));
    stdout.write_all(result.as_slice())?;
    Ok(())
}
