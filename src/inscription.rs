use {
  super::*,
  bitcoin::{
<<<<<<< HEAD
    blockdata::{opcodes, script},
    Script,
=======
    blockdata::{
      opcodes,
      script::{self, Instruction, Instructions, PushBytesBuf},
    },
    taproot::TAPROOT_ANNEX_PREFIX,
    ScriptBuf, Witness,
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
  },
  std::str,
};

<<<<<<< HEAD
const PROTOCOL_ID: &[u8] = b"ord";
=======
const PROTOCOL_ID: [u8; 3] = *b"ord";
const BODY_TAG: [u8; 0] = [];
const CONTENT_TYPE_TAG: [u8; 1] = [1];
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Curse {
  NotInFirstInput,
  NotAtOffsetZero,
  Reinscription,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Inscription {
  body: Option<Vec<u8>>,
  content_type: Option<Vec<u8>>,
}

<<<<<<< HEAD
#[derive(Debug, PartialEq)]
pub(crate) enum ParsedInscription {
  None,
  Partial,
  Complete(Inscription),
=======
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct TransactionInscription {
  pub(crate) inscription: Inscription,
  pub(crate) tx_in_index: u32,
  pub(crate) tx_in_offset: u32,
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
}

impl Inscription {
  #[cfg(test)]
  pub(crate) fn new(content_type: Option<Vec<u8>>, body: Option<Vec<u8>>) -> Self {
    Self { content_type, body }
  }

<<<<<<< HEAD
  pub(crate) fn from_transactions(txs: Vec<Transaction>) -> ParsedInscription {
    let mut sig_scripts = Vec::with_capacity(txs.len());
    for i in 0..txs.len() {
      if txs[i].input.is_empty() {
        return ParsedInscription::None;
      }
      sig_scripts.push(txs[i].input[0].script_sig.clone());
    }
    InscriptionParser::parse(sig_scripts)
=======
  pub(crate) fn from_transaction(tx: &Transaction) -> Vec<TransactionInscription> {
    let mut result = Vec::new();
    for (index, tx_in) in tx.input.iter().enumerate() {
      let Ok(inscriptions) = InscriptionParser::parse(&tx_in.witness) else { continue };

      result.extend(
        inscriptions
          .into_iter()
          .enumerate()
          .map(|(offset, inscription)| TransactionInscription {
            inscription,
            tx_in_index: u32::try_from(index).unwrap(),
            tx_in_offset: u32::try_from(offset).unwrap(),
          })
          .collect::<Vec<TransactionInscription>>(),
      )
    }

    result
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
  }

  pub(crate) fn from_file(chain: Chain, path: impl AsRef<Path>) -> Result<Self, Error> {
    let path = path.as_ref();

    let body = fs::read(path).with_context(|| format!("io error reading {}", path.display()))?;

    if let Some(limit) = chain.inscription_content_size_limit() {
      let len = body.len();
      if len > limit {
        bail!("content size of {len} bytes exceeds {limit} byte limit for {chain} inscriptions");
      }
    }

    let content_type = Media::content_type_for_path(path)?;

    Ok(Self {
      body: Some(body),
      content_type: Some(content_type.into()),
    })
  }

  fn append_reveal_script_to_builder(&self, mut builder: script::Builder) -> script::Builder {
    builder = builder
      .push_opcode(opcodes::OP_FALSE)
      .push_opcode(opcodes::all::OP_IF)
      .push_slice(PROTOCOL_ID);

<<<<<<< HEAD
    if let Some(content_type) = &self.content_type {
      builder = builder.push_slice(&[1]).push_slice(content_type);
=======
    if let Some(content_type) = self.content_type.clone() {
      builder = builder
        .push_slice(CONTENT_TYPE_TAG)
        .push_slice(PushBytesBuf::try_from(content_type).unwrap());
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    }

    if let Some(body) = &self.body {
      builder = builder.push_slice(&[]);
      for chunk in body.chunks(520) {
        builder = builder.push_slice(PushBytesBuf::try_from(chunk.to_vec()).unwrap());
      }
    }

    builder.push_opcode(opcodes::all::OP_ENDIF)
  }

  pub(crate) fn append_reveal_script(&self, builder: script::Builder) -> ScriptBuf {
    self.append_reveal_script_to_builder(builder).into_script()
  }

  pub(crate) fn media(&self) -> Media {
    if self.body.is_none() {
      return Media::Unknown;
    }

    let Some(content_type) = self.content_type() else {
      return Media::Unknown;
    };

    content_type.parse().unwrap_or(Media::Unknown)
  }

  pub(crate) fn body(&self) -> Option<&[u8]> {
    Some(self.body.as_ref()?)
  }

  pub(crate) fn into_body(self) -> Option<Vec<u8>> {
    self.body
  }

  pub(crate) fn content_length(&self) -> Option<usize> {
    Some(self.body()?.len())
  }

  pub(crate) fn content_type(&self) -> Option<&str> {
    str::from_utf8(self.content_type.as_ref()?).ok()
  }

  #[cfg(test)]
  pub(crate) fn to_witness(&self) -> Witness {
    let builder = script::Builder::new();

    let script = self.append_reveal_script(builder);

    let mut witness = Witness::new();

    witness.push(script);
    witness.push([]);

    witness
  }
}

<<<<<<< HEAD
struct InscriptionParser {}
=======
#[derive(Debug, PartialEq)]
pub(crate) enum InscriptionError {
  EmptyWitness,
  InvalidInscription,
  KeyPathSpend,
  NoInscription,
  Script(script::Error),
  UnrecognizedEvenField,
}
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8

impl InscriptionParser {
  fn parse(sig_scripts: Vec<Script>) -> ParsedInscription {
    let sig_script = &sig_scripts[0];

<<<<<<< HEAD
    let mut push_datas_vec = match Self::decode_push_datas(sig_script) {
      Some(push_datas) => push_datas,
      None => return ParsedInscription::None,
    };

    let mut push_datas = push_datas_vec.as_slice();

    // read protocol

    if push_datas.len() < 3 {
      return ParsedInscription::None;
=======
#[derive(Debug)]
struct InscriptionParser<'a> {
  instructions: Peekable<Instructions<'a>>,
}

impl<'a> InscriptionParser<'a> {
  fn parse(witness: &Witness) -> Result<Vec<Inscription>> {
    if witness.is_empty() {
      return Err(InscriptionError::EmptyWitness);
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    }

    let protocol = &push_datas[0];

    if protocol != PROTOCOL_ID {
      return ParsedInscription::None;
    }

    // read npieces

    let mut npieces = match Self::push_data_to_number(&push_datas[1]) {
      Some(n) => n,
      None => return ParsedInscription::None,
    };

    if npieces == 0 {
      return ParsedInscription::None;
    }

    // read content type

<<<<<<< HEAD
    let content_type = push_datas[2].clone();

    push_datas = &push_datas[3..];

    // read body

    let mut body = vec![];

    let mut sig_scripts = sig_scripts.as_slice();

    // loop over transactions
    loop {
      // loop over chunks
      loop {
        if npieces == 0 {
          let inscription = Inscription {
            content_type: Some(content_type),
            body: Some(body),
          };

          return ParsedInscription::Complete(inscription);
        }

        if push_datas.len() < 2 {
          break;
        }

        let next = match Self::push_data_to_number(&push_datas[0]) {
          Some(n) => n,
          None => break,
        };

        if next != npieces - 1 {
          break;
        }

        body.append(&mut push_datas[1].clone());

        push_datas = &push_datas[2..];
        npieces -= 1;
      }

      if sig_scripts.len() <= 1 {
        return ParsedInscription::Partial;
      }

      sig_scripts = &sig_scripts[1..];

      push_datas_vec = match Self::decode_push_datas(&sig_scripts[0]) {
        Some(push_datas) => push_datas,
        None => return ParsedInscription::None,
      };

      if push_datas_vec.len() < 2 {
        return ParsedInscription::None;
      }

      let next = match Self::push_data_to_number(&push_datas_vec[0]) {
        Some(n) => n,
        None => return ParsedInscription::None,
      };

      if next != npieces - 1 {
        return ParsedInscription::None;
      }

      push_datas = push_datas_vec.as_slice();
    }
  }

  fn decode_push_datas(script: &Script) -> Option<Vec<Vec<u8>>> {
    let mut bytes = script.as_bytes();
    let mut push_datas = vec![];

    while !bytes.is_empty() {
      // op_0
      if bytes[0] == 0 {
        push_datas.push(vec![]);
        bytes = &bytes[1..];
        continue;
=======
    InscriptionParser {
      instructions: ScriptBuf::from(Vec::from(script)).instructions().peekable(),
    }
    .parse_inscriptions()
    .into_iter()
    .collect()
  }

  fn parse_inscriptions(&mut self) -> Vec<Result<Inscription>> {
    let mut inscriptions = Vec::new();
    loop {
      let current = self.parse_one_inscription();
      if current == Err(InscriptionError::NoInscription) {
        break;
      }
      inscriptions.push(current);
    }

    inscriptions
  }

  fn parse_one_inscription(&mut self) -> Result<Inscription> {
    self.advance_into_inscription_envelope()?;
    let mut fields = BTreeMap::new();

    loop {
      match self.advance()? {
        Instruction::PushBytes(tag) if tag.as_bytes() == BODY_TAG.as_slice() => {
          let mut body = Vec::new();
          while !self.accept(&Instruction::Op(opcodes::all::OP_ENDIF))? {
            body.extend_from_slice(self.expect_push()?);
          }
          fields.insert(BODY_TAG.as_slice(), body);
          break;
        }
        Instruction::PushBytes(tag) => {
          if fields.contains_key(tag.as_bytes()) {
            return Err(InscriptionError::InvalidInscription);
          }
          fields.insert(tag.as_bytes(), self.expect_push()?.to_vec());
        }
        Instruction::Op(opcodes::all::OP_ENDIF) => break,
        _ => return Err(InscriptionError::InvalidInscription),
      }
    }

    let body = fields.remove(BODY_TAG.as_slice());
    let content_type = fields.remove(CONTENT_TYPE_TAG.as_slice());

    for tag in fields.keys() {
      if let Some(lsb) = tag.first() {
        if lsb % 2 == 0 {
          return Err(InscriptionError::UnrecognizedEvenField);
        }
      }
    }

    Ok(Inscription { body, content_type })
  }

  fn advance(&mut self) -> Result<Instruction<'a>> {
    self
      .instructions
      .next()
      .ok_or(InscriptionError::NoInscription)?
      .map_err(InscriptionError::Script)
  }

  fn advance_into_inscription_envelope(&mut self) -> Result<()> {
    loop {
      if self.match_instructions(&[
        Instruction::PushBytes((&[]).into()), // represents an OF_FALSE
        Instruction::Op(opcodes::all::OP_IF),
        Instruction::PushBytes((&PROTOCOL_ID).into()),
      ])? {
        break;
      }
    }

    Ok(())
  }

  fn match_instructions(&mut self, instructions: &[Instruction]) -> Result<bool> {
    for instruction in instructions {
      if &self.advance()? != instruction {
        return Ok(false);
      }
    }

    Ok(true)
  }

  fn expect_push(&mut self) -> Result<&'a [u8]> {
    match self.advance()? {
      Instruction::PushBytes(bytes) => Ok(bytes.as_bytes()),
      _ => Err(InscriptionError::InvalidInscription),
    }
  }

  fn accept(&mut self, instruction: &Instruction) -> Result<bool> {
    match self.instructions.peek() {
      Some(Ok(next)) => {
        if next == instruction {
          self.advance()?;
          Ok(true)
        } else {
          Ok(false)
        }
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
      }

      // op_1 - op_16
      if bytes[0] >= 81 && bytes[0] <= 96 {
        push_datas.push(vec![bytes[0] - 80]);
        bytes = &bytes[1..];
        continue;
      }

      // op_push 1-75
      if bytes[0] >= 1 && bytes[0] <= 75 {
        let len = bytes[0] as usize;
        if bytes.len() < 1 + len {
          return None;
        }
        push_datas.push(bytes[1..1 + len].to_vec());
        bytes = &bytes[1 + len..];
        continue;
      }

      // op_pushdata1
      if bytes[0] == 76 {
        if bytes.len() < 2 {
          return None;
        }
        let len = bytes[1] as usize;
        if bytes.len() < 2 + len {
          return None;
        }
        push_datas.push(bytes[2..2 + len].to_vec());
        bytes = &bytes[2 + len..];
        continue;
      }

      // op_pushdata2
      if bytes[0] == 77 {
        if bytes.len() < 3 {
          return None;
        }
        let len = ((bytes[1] as usize) << 8) + ((bytes[0] as usize) << 0);
        if bytes.len() < 3 + len {
          return None;
        }
        push_datas.push(bytes[3..3 + len].to_vec());
        bytes = &bytes[3 + len..];
        continue;
      }

      // op_pushdata4
      if bytes[0] == 78 {
        if bytes.len() < 5 {
          return None;
        }
        let len = ((bytes[3] as usize) << 24)
          + ((bytes[2] as usize) << 16)
          + ((bytes[1] as usize) << 8)
          + ((bytes[0] as usize) << 0);
        if bytes.len() < 5 + len {
          return None;
        }
        push_datas.push(bytes[5..5 + len].to_vec());
        bytes = &bytes[5 + len..];
        continue;
      }

      return None;
    }

    Some(push_datas)
  }

  fn push_data_to_number(data: &[u8]) -> Option<u64> {
    if data.len() == 0 {
      return Some(0);
    }

    if data.len() > 8 {
      return None;
    }

    let mut n: u64 = 0;
    let mut m: u64 = 0;

    for i in 0..data.len() {
      n += (data[i] as u64) << m;
      m += 8;
    }

    return Some(n);
  }
}

#[cfg(test)]
mod tests {
  use bitcoin::hashes::hex::FromHex;

  use super::*;

  #[test]
  fn empty() {
    assert_eq!(
<<<<<<< HEAD
      InscriptionParser::parse(vec![Script::new()]),
      ParsedInscription::None
=======
      InscriptionParser::parse(&Witness::new()),
      Err(InscriptionError::EmptyWitness)
    );
  }

  #[test]
  fn ignore_key_path_spends() {
    assert_eq!(
      InscriptionParser::parse(&Witness::from_slice(&[Vec::new()])),
      Err(InscriptionError::KeyPathSpend),
    );
  }

  #[test]
  fn ignore_key_path_spends_with_annex() {
    assert_eq!(
      InscriptionParser::parse(&Witness::from_slice(&[Vec::new(), vec![0x50]])),
      Err(InscriptionError::KeyPathSpend),
    );
  }

  #[test]
  fn ignore_unparsable_scripts() {
    assert_eq!(
      InscriptionParser::parse(&Witness::from_slice(&[vec![0x01], Vec::new()])),
      Err(InscriptionError::Script(script::Error::EarlyEndOfScript)),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
  fn no_inscription() {
    assert_eq!(
<<<<<<< HEAD
      InscriptionParser::parse(vec![Script::from_hex("483045022100a942753a4e036f59648469cb6ac19b33b1e423ff5ceaf93007001b54df46ca1f022025f6554a58b6fde5ff24b5e2556acc57d1d2108c0de2a14096e7ddae9c9fb96d0121034523d20080d1abe75a9fbed07b83e695db2f30e2cd89b80b154a0ed70badfc90").unwrap()]),
      ParsedInscription::None
=======
      InscriptionParser::parse(&Witness::from_slice(&[
        ScriptBuf::new().into_bytes(),
        Vec::new()
      ])),
      Ok(vec![])
    );
  }

  #[test]
  fn duplicate_field() {
    assert_eq!(
      InscriptionParser::parse(&envelope(&[
        b"ord",
        &[1],
        b"text/plain;charset=utf-8",
        &[1],
        b"text/plain;charset=utf-8",
        &[],
        b"ord",
      ])),
      Err(InscriptionError::InvalidInscription),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
  fn valid() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[0]);
    script.push(&[4]);
    script.push(b"woof");
    assert_eq!(
<<<<<<< HEAD
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Complete(inscription("text/plain;charset=utf-8", "woof"))
=======
      InscriptionParser::parse(&envelope(&[
        b"ord",
        &[1],
        b"text/plain;charset=utf-8",
        &[],
        b"ord",
      ])),
      Ok(vec![inscription("text/plain;charset=utf-8", "ord")]),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
  fn valid_empty_fields() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[81]);
    script.push(&[0]);
    script.push(&[0]);
    script.push(&[0]);
    assert_eq!(
<<<<<<< HEAD
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Complete(inscription("", ""))
=======
      InscriptionParser::parse(&envelope(&[
        b"ord",
        &[1],
        b"text/plain;charset=utf-8",
        &[3],
        b"bar",
        &[],
        b"ord",
      ])),
      Ok(vec![inscription("text/plain;charset=utf-8", "ord")]),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
  fn valid_multipart() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[82]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[81]);
    script.push(&[4]);
    script.push(b"woof");
    script.push(&[0]);
    script.push(&[5]);
    script.push(b" woof");
    assert_eq!(
<<<<<<< HEAD
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Complete(inscription("text/plain;charset=utf-8", "woof woof"))
    );
  }

  #[test]
  fn valid_multitx() {
    let mut script1: Vec<&[u8]> = Vec::new();
    let mut script2: Vec<&[u8]> = Vec::new();
    script1.push(&[3]);
    script1.push(b"ord");
    script1.push(&[82]);
    script1.push(&[24]);
    script1.push(b"text/plain;charset=utf-8");
    script1.push(&[81]);
    script1.push(&[4]);
    script1.push(b"woof");
    script2.push(&[0]);
    script2.push(&[5]);
    script2.push(b" woof");
    assert_eq!(
      InscriptionParser::parse(vec![
        Script::from(script1.concat()),
        Script::from(script2.concat())
      ]),
      ParsedInscription::Complete(inscription("text/plain;charset=utf-8", "woof woof"))
    );
  }

  #[test]
  fn valid_multitx_long() {
    let mut expected = String::new();
    let mut script_parts = vec![];

    let mut script: Vec<Vec<u8>> = Vec::new();
    script.push(vec![3]);
    script.push(b"ord".to_vec());
    const LEN: usize = 100000;
    push_number(&mut script, LEN as u64);
    script.push(vec![24]);
    script.push(b"text/plain;charset=utf-8".to_vec());

    let mut i = 0;
    while i < LEN {
      let text = format!("{}", i % 10);
      expected += text.as_str();
      push_number(&mut script, (LEN - i - 1) as u64);
      script.push(vec![1]);
      script.push(text.as_bytes().to_vec());
      i += 1;

      let text = format!("{}", i % 10);
      expected += text.as_str();
      push_number(&mut script, (LEN - i - 1) as u64);
      script.push(vec![1]);
      script.push(text.as_bytes().to_vec());
      i += 1;

      script_parts.push(script);
      script = Vec::new();
    }

    let mut scripts = vec![];
    script_parts
      .iter()
      .for_each(|script| scripts.push(Script::from(script.concat())));

    assert_eq!(
      InscriptionParser::parse(scripts),
      ParsedInscription::Complete(inscription("text/plain;charset=utf-8", expected))
    );
  }

  #[test]
  fn valid_multitx_extradata() {
    let mut script1: Vec<&[u8]> = Vec::new();
    let mut script2: Vec<&[u8]> = Vec::new();
    script1.push(&[3]);
    script1.push(b"ord");
    script1.push(&[82]);
    script1.push(&[24]);
    script1.push(b"text/plain;charset=utf-8");
    script1.push(&[81]);
    script1.push(&[4]);
    script1.push(b"woof");
    script1.push(&[82]);
    script1.push(&[4]);
    script1.push(b"bark");
    script2.push(&[0]);
    script2.push(&[5]);
    script2.push(b" woof");
    assert_eq!(
      InscriptionParser::parse(vec![
        Script::from(script1.concat()),
        Script::from(script2.concat())
      ]),
      ParsedInscription::Complete(inscription("text/plain;charset=utf-8", "woof woof"))
    );
  }

  #[test]
  fn invalid_multitx_missingdata() {
    let mut script1: Vec<&[u8]> = Vec::new();
    let mut script2: Vec<&[u8]> = Vec::new();
    script1.push(&[3]);
    script1.push(b"ord");
    script1.push(&[82]);
    script1.push(&[24]);
    script1.push(b"text/plain;charset=utf-8");
    script1.push(&[81]);
    script1.push(&[4]);
    script1.push(b"woof");
    script2.push(&[0]);
    assert_eq!(
      InscriptionParser::parse(vec![
        Script::from(script1.concat()),
        Script::from(script2.concat())
      ]),
      ParsedInscription::None
    );
  }

  #[test]
  fn invalid_multitx_wrongcountdown() {
    let mut script1: Vec<&[u8]> = Vec::new();
    let mut script2: Vec<&[u8]> = Vec::new();
    script1.push(&[3]);
    script1.push(b"ord");
    script1.push(&[82]);
    script1.push(&[24]);
    script1.push(b"text/plain;charset=utf-8");
    script1.push(&[81]);
    script1.push(&[4]);
    script1.push(b"woof");
    script2.push(&[81]);
    script2.push(&[5]);
    script2.push(b" woof");
    assert_eq!(
      InscriptionParser::parse(vec![
        Script::from(script1.concat()),
        Script::from(script2.concat())
      ]),
      ParsedInscription::None
    );
  }

  fn push_number(script: &mut Vec<Vec<u8>>, num: u64) {
    if num == 0 {
      script.push(vec![0]);
      return;
    }

    if num <= 16 {
      script.push(vec![(80 + num) as u8]);
      return;
    }

    if num <= 0x7f {
      script.push(vec![1]);
      script.push(vec![num as u8]);
      return;
    }

    if num <= 0x7fff {
      script.push(vec![2]);
      script.push(vec![(num % 256) as u8, (num / 256) as u8]);
      return;
    }

    if num <= 0x7fffff {
      script.push(vec![3]);
      script.push(vec![
        (num % 256) as u8,
        ((num / 256) % 256) as u8,
        (num / 256 / 256) as u8,
      ]);
      return;
    }

    panic!();
  }

  #[test]
  fn valid_long() {
    let mut expected = String::new();
    let mut script: Vec<Vec<u8>> = Vec::new();
    script.push(vec![3]);
    script.push(b"ord".to_vec());
    const LEN: usize = 100000;
    push_number(&mut script, LEN as u64);
    script.push(vec![24]);
    script.push(b"text/plain;charset=utf-8".to_vec());
    for i in 0..LEN {
      let text = format!("{}", i % 10);
      expected += text.as_str();
      push_number(&mut script, (LEN - i - 1) as u64);
      script.push(vec![1]);
      script.push(text.as_bytes().to_vec());
    }
    assert_eq!(
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Complete(inscription("text/plain;charset=utf-8", expected))
    );
  }

  #[test]
  fn duplicate_field() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[0]);
    script.push(&[4]);
    script.push(b"woof");
    assert_eq!(
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Partial,
    );
  }

  #[test]
  fn invalid_tag() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[82]);
    script.push(&[4]);
    script.push(b"woof");
    assert_eq!(
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Partial,
    );
  }

  #[test]
  fn no_content() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    assert_eq!(
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Partial,
=======
      InscriptionParser::parse(&envelope(&[b"ord", &[1], b"text/plain;charset=utf-8"])),
      Ok(vec![Inscription {
        content_type: Some(b"text/plain;charset=utf-8".to_vec()),
        body: None,
      }]),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
  fn no_content_type() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[0]);
    script.push(&[4]);
    script.push(b"woof");
    assert_eq!(
<<<<<<< HEAD
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::None,
=======
      InscriptionParser::parse(&envelope(&[b"ord", &[], b"foo"])),
      Ok(vec![Inscription {
        content_type: None,
        body: Some(b"foo".to_vec()),
      }]),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
  fn valid_with_extra_data() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[0]);
    script.push(&[4]);
    script.push(b"woof");
    script.push(&[9]);
    script.push(b"woof woof");
    script.push(&[14]);
    script.push(b"woof woof woof");
    assert_eq!(
<<<<<<< HEAD
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Complete(inscription("text/plain;charset=utf-8", "woof"))
=======
      InscriptionParser::parse(&envelope(&[
        b"ord",
        &[1],
        b"text/plain;charset=utf-8",
        &[],
        b"foo",
        b"bar"
      ])),
      Ok(vec![inscription("text/plain;charset=utf-8", "foobar")]),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
  fn prefix_data() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[4]);
    script.push(b"woof");
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[0]);
    script.push(&[4]);
    script.push(b"woof");
    assert_eq!(
<<<<<<< HEAD
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::None,
=======
      InscriptionParser::parse(&envelope(&[b"ord", &[1], b"text/plain;charset=utf-8", &[]])),
      Ok(vec![inscription("text/plain;charset=utf-8", "")]),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
  fn wrong_protocol() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"dog");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[0]);
    script.push(&[4]);
    script.push(b"woof");
    assert_eq!(
<<<<<<< HEAD
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::None
=======
      InscriptionParser::parse(&envelope(&[
        b"ord",
        &[1],
        b"text/plain;charset=utf-8",
        &[],
        &[],
        &[],
        &[],
        &[],
        &[],
      ])),
      Ok(vec![inscription("text/plain;charset=utf-8", "")]),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
<<<<<<< HEAD
  fn incomplete_multipart() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[82]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[81]);
    script.push(&[4]);
    script.push(b"woof");
    assert_eq!(
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Partial
=======
  fn valid_ignore_trailing() {
    let script = script::Builder::new()
      .push_opcode(opcodes::OP_FALSE)
      .push_opcode(opcodes::all::OP_IF)
      .push_slice(b"ord")
      .push_slice([1])
      .push_slice(b"text/plain;charset=utf-8")
      .push_slice([])
      .push_slice(b"ord")
      .push_opcode(opcodes::all::OP_ENDIF)
      .push_opcode(opcodes::all::OP_CHECKSIG)
      .into_script();

    assert_eq!(
      InscriptionParser::parse(&Witness::from_slice(&[script.into_bytes(), Vec::new()])),
      Ok(vec![inscription("text/plain;charset=utf-8", "ord")]),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
<<<<<<< HEAD
  fn bad_npieces() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[82]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[83]);
    script.push(&[4]);
    script.push(b"woof");
    script.push(&[0]);
    script.push(&[4]);
    script.push(b"woof");
    assert_eq!(
      InscriptionParser::parse(vec![Script::from(script.concat())]),
      ParsedInscription::Partial
    );
=======
  fn valid_ignore_preceding() {
    let script = script::Builder::new()
      .push_opcode(opcodes::all::OP_CHECKSIG)
      .push_opcode(opcodes::OP_FALSE)
      .push_opcode(opcodes::all::OP_IF)
      .push_slice(b"ord")
      .push_slice([1])
      .push_slice(b"text/plain;charset=utf-8")
      .push_slice([])
      .push_slice(b"ord")
      .push_opcode(opcodes::all::OP_ENDIF)
      .into_script();

    assert_eq!(
      InscriptionParser::parse(&Witness::from_slice(&[script.into_bytes(), Vec::new()])),
      Ok(vec![inscription("text/plain;charset=utf-8", "ord")]),
    );
  }

  #[test]
  fn do_not_ignore_inscriptions_after_first() {
    let script = script::Builder::new()
      .push_opcode(opcodes::OP_FALSE)
      .push_opcode(opcodes::all::OP_IF)
      .push_slice(b"ord")
      .push_slice([1])
      .push_slice(b"text/plain;charset=utf-8")
      .push_slice([])
      .push_slice(b"foo")
      .push_opcode(opcodes::all::OP_ENDIF)
      .push_opcode(opcodes::OP_FALSE)
      .push_opcode(opcodes::all::OP_IF)
      .push_slice(b"ord")
      .push_slice([1])
      .push_slice(b"text/plain;charset=utf-8")
      .push_slice([])
      .push_slice(b"bar")
      .push_opcode(opcodes::all::OP_ENDIF)
      .into_script();

    assert_eq!(
      InscriptionParser::parse(&Witness::from_slice(&[script.into_bytes(), Vec::new()])),
      Ok(vec![
        inscription("text/plain;charset=utf-8", "foo"),
        inscription("text/plain;charset=utf-8", "bar")
      ]),
    );
  }

  #[test]
  fn invalid_utf8_does_not_render_inscription_invalid() {
    assert_eq!(
      InscriptionParser::parse(&envelope(&[
        b"ord",
        &[1],
        b"text/plain;charset=utf-8",
        &[],
        &[0b10000000]
      ])),
      Ok(vec![inscription("text/plain;charset=utf-8", [0b10000000])]),
    );
  }

  #[test]
  fn no_endif() {
    let script = script::Builder::new()
      .push_opcode(opcodes::OP_FALSE)
      .push_opcode(opcodes::all::OP_IF)
      .push_slice(b"ord")
      .into_script();

    assert_eq!(
      InscriptionParser::parse(&Witness::from_slice(&[script.into_bytes(), Vec::new()])),
      Ok(vec![])
    );
  }

  #[test]
  fn no_op_false() {
    let script = script::Builder::new()
      .push_opcode(opcodes::all::OP_IF)
      .push_slice(b"ord")
      .push_opcode(opcodes::all::OP_ENDIF)
      .into_script();

    assert_eq!(
      InscriptionParser::parse(&Witness::from_slice(&[script.into_bytes(), Vec::new()])),
      Ok(vec![])
    );
  }

  #[test]
  fn empty_envelope() {
    assert_eq!(InscriptionParser::parse(&envelope(&[])), Ok(vec![]));
  }

  #[test]
  fn wrong_magic_number() {
    assert_eq!(InscriptionParser::parse(&envelope(&[b"foo"])), Ok(vec![]));
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
  }

  #[test]
  fn extract_from_transaction() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[0]);
    script.push(&[4]);
    script.push(b"woof");

    let tx = Transaction {
      version: 0,
      lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
      input: vec![TxIn {
        previous_output: OutPoint::null(),
<<<<<<< HEAD
        script_sig: Script::from(script.concat()),
=======
        script_sig: ScriptBuf::new(),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
        sequence: Sequence(0),
        witness: Witness::new(),
      }],
      output: Vec::new(),
    };

    assert_eq!(
<<<<<<< HEAD
      Inscription::from_transactions(vec![tx]),
      ParsedInscription::Complete(inscription("text/plain;charset=utf-8", "woof")),
=======
      Inscription::from_transaction(&tx),
      vec![transaction_inscription(
        "text/plain;charset=utf-8",
        "ord",
        0,
        0
      )],
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  #[test]
<<<<<<< HEAD
  fn do_not_extract_from_second_input() {
    let mut script: Vec<&[u8]> = Vec::new();
    script.push(&[3]);
    script.push(b"ord");
    script.push(&[81]);
    script.push(&[24]);
    script.push(b"text/plain;charset=utf-8");
    script.push(&[0]);
    script.push(&[4]);
    script.push(b"woof");

=======
  fn extract_from_second_input() {
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    let tx = Transaction {
      version: 0,
      lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
      input: vec![
        TxIn {
          previous_output: OutPoint::null(),
          script_sig: ScriptBuf::new(),
          sequence: Sequence(0),
          witness: Witness::new(),
        },
        TxIn {
          previous_output: OutPoint::null(),
<<<<<<< HEAD
          script_sig: Script::from(script.concat()),
=======
          script_sig: ScriptBuf::new(),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
          sequence: Sequence(0),
          witness: Witness::new(),
        },
      ],
      output: Vec::new(),
    };

<<<<<<< HEAD
    assert_eq!(
      Inscription::from_transactions(vec![tx]),
      ParsedInscription::None
=======
    assert_eq!(
      Inscription::from_transaction(&tx),
      vec![transaction_inscription("foo", [1; 1040], 1, 0)]
    );
  }

  #[test]
  fn extract_from_second_envelope() {
    let mut builder = script::Builder::new();
    builder = inscription("foo", [1; 100]).append_reveal_script_to_builder(builder);
    builder = inscription("bar", [1; 100]).append_reveal_script_to_builder(builder);

    let witness = Witness::from_slice(&[builder.into_script().into_bytes(), Vec::new()]);

    let tx = Transaction {
      version: 0,
      lock_time: bitcoin::locktime::absolute::LockTime::ZERO,
      input: vec![TxIn {
        previous_output: OutPoint::null(),
        script_sig: ScriptBuf::new(),
        sequence: Sequence(0),
        witness,
      }],
      output: Vec::new(),
    };

    assert_eq!(
      Inscription::from_transaction(&tx),
      vec![
        transaction_inscription("foo", [1; 100], 0, 0),
        transaction_inscription("bar", [1; 100], 0, 1)
      ]
    );
  }

  #[test]
  fn inscribe_png() {
    assert_eq!(
      InscriptionParser::parse(&envelope(&[b"ord", &[1], b"image/png", &[], &[1; 100]])),
      Ok(vec![inscription("image/png", [1; 100])]),
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    );
  }

  /*
  #[test]
  fn reveal_script_chunks_data() {
    assert_eq!(
      inscription("foo", [])
        .append_reveal_script(script::Builder::new())
        .instructions()
        .count(),
      7
    );

    assert_eq!(
      inscription("foo", [0; 1])
        .append_reveal_script(script::Builder::new())
        .instructions()
        .count(),
      8
    );

    assert_eq!(
      inscription("foo", [0; 520])
        .append_reveal_script(script::Builder::new())
        .instructions()
        .count(),
      8
    );

    assert_eq!(
      inscription("foo", [0; 521])
        .append_reveal_script(script::Builder::new())
        .instructions()
        .count(),
      9
    );

    assert_eq!(
      inscription("foo", [0; 1040])
        .append_reveal_script(script::Builder::new())
        .instructions()
        .count(),
      9
    );

    assert_eq!(
      inscription("foo", [0; 1041])
        .append_reveal_script(script::Builder::new())
        .instructions()
        .count(),
      10
    );
  }
<<<<<<< HEAD
  */
=======

  #[test]
  fn chunked_data_is_parsable() {
    let mut witness = Witness::new();

    witness.push(&inscription("foo", [1; 1040]).append_reveal_script(script::Builder::new()));

    witness.push([]);

    assert_eq!(
      InscriptionParser::parse(&witness).unwrap(),
      vec![inscription("foo", [1; 1040])],
    );
  }

  #[test]
  fn round_trip_with_no_fields() {
    let mut witness = Witness::new();

    witness.push(
      &Inscription {
        content_type: None,
        body: None,
      }
      .append_reveal_script(script::Builder::new()),
    );

    witness.push([]);

    assert_eq!(
      InscriptionParser::parse(&witness).unwrap(),
      vec![Inscription {
        content_type: None,
        body: None,
      }]
    );
  }

  #[test]
  fn unknown_odd_fields_are_ignored() {
    assert_eq!(
      InscriptionParser::parse(&envelope(&[b"ord", &[3], &[0]])),
      Ok(vec![Inscription {
        content_type: None,
        body: None,
      }]),
    );
  }

  #[test]
  fn unknown_even_fields_are_invalid() {
    assert_eq!(
      InscriptionParser::parse(&envelope(&[b"ord", &[2], &[0]])),
      Err(InscriptionError::UnrecognizedEvenField),
    );
  }
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
}
