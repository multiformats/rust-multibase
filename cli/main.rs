use async_std::{
    io::{self, Read, Write},
    task,
};
use core::{fmt, str::FromStr};
use exitfailure::ExitFailure;
use failure::{format_err, Error};
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
        #[structopt(short = "b", long = "base", default_value = "base58-btc")]
        base: StrBase,
    },
    #[structopt(name = "decode")]
    Decode,
}

fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    task::block_on(async {
        let opts = Opts::from_args();
        match opts.mode {
            Mode::Encode { base } => encode(base).await,
            Mode::Decode => decode().await,
        }
    })
}

#[derive(Debug)]
struct StrBase(Base);

impl fmt::Display for StrBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base_str = match self.0 {
            Base::Base2 => "base2",
            Base::Base8 => "base8",
            Base::Base10 => "base10",
            Base::Base16 => "base16",
            Base::Base16Upper => "base16-upper",
            Base::Base32 => "base32",
            Base::Base32Upper => "base32-upper",
            Base::Base32hex => "base32-hex",
            Base::Base32hexUpper => "base32-hex-upper",
            Base::Base32z => "base32-z",
            Base::Base58flickr => "base58-flickr",
            Base::Base58btc => "base58-btc",
            Base::Base64 => "base64",
            Base::Base64url => "base64-url",
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
            "base16" => Ok(Base::Base16),
            "base16-upper" => Ok(Base::Base16Upper),
            "base32" => Ok(Base::Base32),
            "base32-upper" => Ok(Base::Base32Upper),
            "base32-hex" => Ok(Base::Base32hex),
            "base32-hex-upper" => Ok(Base::Base32hexUpper),
            "base32-z" => Ok(Base::Base32z),
            "base58-flickr" => Ok(Base::Base58flickr),
            "base58-btc" => Ok(Base::Base58btc),
            "base64" => Ok(Base::Base64),
            "base64-url" => Ok(Base::Base64url),
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

async fn encode(base: StrBase) -> Result<(), ExitFailure> {
    log::debug!("encoding with {}", base);
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer).await?;
    log::debug!("read {:?} from stdin", buffer);
    let result = multibase::encode(base.into(), buffer.as_slice());
    stdout.write_all(result.as_bytes()).await?;
    Ok(())
}

async fn decode() -> Result<(), ExitFailure> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).await?;
    log::debug!("read {:?} from stdin", buffer);
    let (base, result) = multibase::decode(buffer.as_str())?;
    log::debug!("detected {}", StrBase(base));
    stdout.write_all(result.as_slice()).await?;
    Ok(())
}
