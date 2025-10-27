use crate::{Inscription, InscriptionId};
use bitcoin::Txid;
use serde::{Deserialize, Serialize};

pub const SUFFIX: &str = ".pepemap";
pub const MAX_NUMBER: u32 = 5_000_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepemapEntry {
  pub number: u32,
  pub inscription_id: InscriptionId,
  pub txid: Txid,
  pub owner: String,
  pub block_height: u32,
  pub block_time: u32,
}

pub fn parse_number(inscription: &Inscription) -> Option<u32> {
  let body = inscription.body()?;
  let body_str = std::str::from_utf8(body).ok()?.trim();
  let Some(number_str) = body_str.strip_suffix(SUFFIX) else {
    return None;
  };

  if number_str.is_empty() {
    return None;
  }

  let number = number_str.parse::<u32>().ok()?;
  if number == 0 || number > MAX_NUMBER {
    return None;
  }

  Some(number)
}
