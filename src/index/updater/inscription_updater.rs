<<<<<<< HEAD
use super::*;
use crate::inscription::ParsedInscription;
=======
use {super::*, inscription::Curse};
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8

#[derive(Debug, Clone)]
pub(super) struct Flotsam {
  inscription_id: InscriptionId,
  offset: u64,
  origin: Origin,
}

#[derive(Debug, Clone)]
enum Origin {
  New {
    fee: u64,
    cursed: bool,
    unbound: bool,
  },
  Old {
    old_satpoint: SatPoint,
  },
}

pub(super) struct InscriptionUpdater<'a, 'db, 'tx> {
  flotsam: Vec<Flotsam>,
  height: u64,
  id_to_satpoint: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, &'static SatPointValue>,
  id_to_txids: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, &'static [u8]>,
  txid_to_tx: &'a mut Table<'db, 'tx, &'static [u8], &'static [u8]>,
  partial_txid_to_txids: &'a mut Table<'db, 'tx, &'static [u8], &'static [u8]>,
  value_receiver: &'a mut Receiver<u64>,
  id_to_entry: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, InscriptionEntryValue>,
  pub(super) lost_sats: u64,
  next_cursed_number: i64,
  next_number: i64,
  number_to_id: &'a mut Table<'db, 'tx, i64, &'static InscriptionIdValue>,
  outpoint_to_value: &'a mut Table<'db, 'tx, &'static OutPointValue, u64>,
  reward: u64,
<<<<<<< HEAD
  sat_to_inscription_id: &'a mut Table<'db, 'tx, u128, &'static InscriptionIdValue>,
  satpoint_to_id: &'a mut Table<'db, 'tx, &'static SatPointValue, &'static InscriptionIdValue>,
=======
  reinscription_id_to_seq_num: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, u64>,
  sat_to_inscription_id: &'a mut MultimapTable<'db, 'tx, u64, &'static InscriptionIdValue>,
  satpoint_to_id:
    &'a mut MultimapTable<'db, 'tx, &'static SatPointValue, &'static InscriptionIdValue>,
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
  timestamp: u32,
  pub(super) unbound_inscriptions: u64,
  value_cache: &'a mut HashMap<OutPoint, u64>,
}

impl<'a, 'db, 'tx> InscriptionUpdater<'a, 'db, 'tx> {
  pub(super) fn new(
    height: u64,
    id_to_satpoint: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, &'static SatPointValue>,
    id_to_txids: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, &'static [u8]>,
    txid_to_tx: &'a mut Table<'db, 'tx, &'static [u8], &'static [u8]>,
    partial_txid_to_txids: &'a mut Table<'db, 'tx, &'static [u8], &'static [u8]>,
    value_receiver: &'a mut Receiver<u64>,
    id_to_entry: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, InscriptionEntryValue>,
    lost_sats: u64,
    number_to_id: &'a mut Table<'db, 'tx, i64, &'static InscriptionIdValue>,
    outpoint_to_value: &'a mut Table<'db, 'tx, &'static OutPointValue, u64>,
<<<<<<< HEAD
    sat_to_inscription_id: &'a mut Table<'db, 'tx, u128, &'static InscriptionIdValue>,
    satpoint_to_id: &'a mut Table<'db, 'tx, &'static SatPointValue, &'static InscriptionIdValue>,
=======
    reinscription_id_to_seq_num: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, u64>,
    sat_to_inscription_id: &'a mut MultimapTable<'db, 'tx, u64, &'static InscriptionIdValue>,
    satpoint_to_id: &'a mut MultimapTable<
      'db,
      'tx,
      &'static SatPointValue,
      &'static InscriptionIdValue,
    >,
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    timestamp: u32,
    unbound_inscriptions: u64,
    value_cache: &'a mut HashMap<OutPoint, u64>,
  ) -> Result<Self> {
    let next_cursed_number = number_to_id
      .iter()?
      .next()
      .and_then(|result| result.ok())
      .map(|(number, _id)| number.value() - 1)
      .unwrap_or(-1);

    let next_number = number_to_id
      .iter()?
      .next_back()
      .and_then(|result| result.ok())
      .map(|(number, _id)| number.value() + 1)
      .unwrap_or(0);

    Ok(Self {
      flotsam: Vec::new(),
      height,
      id_to_satpoint,
      id_to_txids,
      txid_to_tx,
      partial_txid_to_txids,
      value_receiver,
      id_to_entry,
      lost_sats,
      next_cursed_number,
      next_number,
      number_to_id,
      outpoint_to_value,
      reward: Height(height).subsidy(),
      reinscription_id_to_seq_num,
      sat_to_inscription_id,
      satpoint_to_id,
      timestamp,
      unbound_inscriptions,
      value_cache,
    })
  }

  pub(super) fn index_transaction_inscriptions(
    &mut self,
    tx: &Transaction,
    txid: Txid,
<<<<<<< HEAD
    input_sat_ranges: Option<&VecDeque<(u128, u128)>>,
  ) -> Result<u64> {
    let mut inscriptions = Vec::new();

=======
    input_sat_ranges: Option<&VecDeque<(u64, u64)>>,
  ) -> Result {
    let mut new_inscriptions = Inscription::from_transaction(tx).into_iter().peekable();
    let mut floating_inscriptions = Vec::new();
    let mut inscribed_offsets = BTreeMap::new();
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    let mut input_value = 0;
    let mut id_counter = 0;

    for (input_index, tx_in) in tx.input.iter().enumerate() {
      // skip subsidy since no inscriptions possible
      if tx_in.previous_output.is_null() {
        input_value += Height(self.height).subsidy();
        continue;
      }

      // find existing inscriptions on input (transfers of inscriptions)
      for (old_satpoint, inscription_id) in Index::inscriptions_on_output_ordered(
        self.reinscription_id_to_seq_num,
        self.satpoint_to_id,
        tx_in.previous_output,
      )? {
        let offset = input_value + old_satpoint.offset;
        floating_inscriptions.push(Flotsam {
          offset,
          inscription_id,
          origin: Origin::Old { old_satpoint },
        });

        inscribed_offsets
          .entry(offset)
          .and_modify(|(_id, count)| *count += 1)
          .or_insert((inscription_id, 0));
      }

      let offset = input_value;

      // multi-level cache for UTXO set to get to the input amount
      input_value += if let Some(value) = self.value_cache.remove(&tx_in.previous_output) {
        value
      } else if let Some(value) = self
        .outpoint_to_value
        .remove(&tx_in.previous_output.store())?
      {
        value.value()
      } else {
        self.value_receiver.blocking_recv().ok_or_else(|| {
          anyhow!(
            "failed to get transaction for {}",
            tx_in.previous_output.txid
          )
        })?
      };

      // go through all inscriptions in this input
      while let Some(inscription) = new_inscriptions.peek() {
        if inscription.tx_in_index != u32::try_from(input_index).unwrap() {
          break;
        }

        let inscription_id = InscriptionId {
          txid,
          index: id_counter,
        };

        let curse = if inscription.tx_in_index != 0 {
          Some(Curse::NotInFirstInput)
        } else if inscription.tx_in_offset != 0 {
          Some(Curse::NotAtOffsetZero)
        } else if inscribed_offsets.contains_key(&offset) {
          let seq_num = self.reinscription_id_to_seq_num.len()?;

          let sat = Self::calculate_sat(input_sat_ranges, offset);
          log::info!("processing reinscription {inscription_id} on sat {:?}: sequence number {seq_num}, inscribed offsets {:?}", sat, inscribed_offsets);

          // if reinscription track its ordering
          self
            .reinscription_id_to_seq_num
            .insert(&inscription_id.store(), seq_num)?;

          Some(Curse::Reinscription)
        } else {
          None
        };

        if curse.is_some() {
          log::info!("found cursed inscription {inscription_id}: {:?}", curse);
        }

        let cursed = if let Some(Curse::Reinscription) = curse {
          let first_reinscription = inscribed_offsets
            .get(&offset)
            .map(|(_id, count)| count == &0)
            .unwrap_or(false);

          let initial_inscription_is_cursed = inscribed_offsets
            .get(&offset)
            .and_then(|(inscription_id, _count)| {
              match self.id_to_entry.get(&inscription_id.store()) {
                Ok(option) => option.map(|entry| {
                  let loaded_entry = InscriptionEntry::load(entry.value());
                  loaded_entry.number < 0
                }),
                Err(_) => None,
              }
            })
            .unwrap_or(false);

          log::info!("{inscription_id}: is first reinscription: {first_reinscription}, initial inscription is cursed: {initial_inscription_is_cursed}");

          !(initial_inscription_is_cursed && first_reinscription)
        } else {
          curse.is_some()
        };

        let unbound = input_value == 0 || inscription.tx_in_offset != 0;

        if curse.is_some() || unbound {
          log::info!(
            "indexing inscription {inscription_id} with curse {:?} as cursed {} and unbound {}",
            curse,
            cursed,
            unbound
          );
        }

        floating_inscriptions.push(Flotsam {
          inscription_id,
          offset,
          origin: Origin::New {
            fee: 0,
            cursed,
            unbound,
          },
        });

        new_inscriptions.next();
        id_counter += 1;
      }
    }

<<<<<<< HEAD
    if inscriptions.iter().all(|flotsam| flotsam.offset != 0) {
      let previous_txid = tx.input[0].previous_output.txid;
      let previous_txid_bytes: [u8; 32] = previous_txid.into_inner();
      let mut txids_vec = vec![];

      let txs = match self
        .partial_txid_to_txids
        .get(&previous_txid_bytes.as_slice())?
      {
        Some(partial_txids) => {
          let txids = partial_txids.value();
          let mut txs = vec![];
          txids_vec = txids.to_vec();
          for i in 0..txids.len() / 32 {
            let txid = &txids[i * 32..i * 32 + 32];
            let tx_result = self.txid_to_tx.get(txid)?;
            let tx_result = tx_result.unwrap();
            let tx_buf = tx_result.value();
            let mut cursor = std::io::Cursor::new(tx_buf);
            let tx = bitcoin::Transaction::consensus_decode(&mut cursor)?;
            txs.push(tx);
          }
          txs.push(tx.clone());
          txs
        }
        None => {
          vec![tx.clone()]
        }
      };

      match Inscription::from_transactions(txs) {
        ParsedInscription::None => {
          // todo: clean up db
        }

        ParsedInscription::Partial => {
          let mut txid_vec = txid.into_inner().to_vec();
          txids_vec.append(&mut txid_vec);

          self
            .partial_txid_to_txids
            .remove(&previous_txid_bytes.as_slice())?;
          self
            .partial_txid_to_txids
            .insert(&txid.into_inner().as_slice(), txids_vec.as_slice())?;

          let mut tx_buf = vec![];
          tx.consensus_encode(&mut tx_buf)?;
          self
            .txid_to_tx
            .insert(&txid.into_inner().as_slice(), tx_buf.as_slice())?;
        }

        ParsedInscription::Complete(_inscription) => {
          self
            .partial_txid_to_txids
            .remove(&previous_txid_bytes.as_slice())?;

          let mut tx_buf = vec![];
          tx.consensus_encode(&mut tx_buf)?;
          self
            .txid_to_tx
            .insert(&txid.into_inner().as_slice(), tx_buf.as_slice())?;

          let mut txid_vec = txid.into_inner().to_vec();
          txids_vec.append(&mut txid_vec);

          let mut inscription_id = [0_u8; 36];
          unsafe {
            std::ptr::copy_nonoverlapping(txids_vec.as_ptr(), inscription_id.as_mut_ptr(), 32)
          }
          self
            .id_to_txids
            .insert(&inscription_id, txids_vec.as_slice())?;

          let og_inscription_id = InscriptionId {
            txid: Txid::from_slice(&txids_vec[0..32]).unwrap(),
            index: 0
          };

          inscriptions.push(Flotsam {
            inscription_id: og_inscription_id,
            offset: 0,
            origin: Origin::New(
              input_value - tx.output.iter().map(|txout| txout.value).sum::<u64>(),
            ),
          });
        }
      }
    };
=======
    // still have to normalize over inscription size
    let total_output_value = tx.output.iter().map(|txout| txout.value).sum::<u64>();
    let mut floating_inscriptions = floating_inscriptions
      .into_iter()
      .map(|flotsam| {
        if let Flotsam {
          inscription_id,
          offset,
          origin:
            Origin::New {
              fee: _,
              cursed,
              unbound,
            },
        } = flotsam
        {
          Flotsam {
            inscription_id,
            offset,
            origin: Origin::New {
              fee: (input_value - total_output_value) / u64::from(id_counter),
              cursed,
              unbound,
            },
          }
        } else {
          flotsam
        }
      })
      .collect::<Vec<Flotsam>>();
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8

    let is_coinbase = tx
      .input
      .first()
      .map(|tx_in| tx_in.previous_output.is_null())
      .unwrap_or_default();

    if is_coinbase {
      floating_inscriptions.append(&mut self.flotsam);
    }

    floating_inscriptions.sort_by_key(|flotsam| flotsam.offset);
    let mut inscriptions = floating_inscriptions.into_iter().peekable();

    let mut output_value = 0;
    for (vout, tx_out) in tx.output.iter().enumerate() {
      let end = output_value + tx_out.value;

      while let Some(flotsam) = inscriptions.peek() {
        if flotsam.offset >= end {
          break;
        }

        let new_satpoint = SatPoint {
          outpoint: OutPoint {
            txid,
            vout: vout.try_into().unwrap(),
          },
          offset: flotsam.offset - output_value,
        };

        self.update_inscription_location(
          input_sat_ranges,
          inscriptions.next().unwrap(),
          new_satpoint,
        )?;
      }

      output_value = end;

      self.value_cache.insert(
        OutPoint {
          vout: vout.try_into().unwrap(),
          txid,
        },
        tx_out.value,
      );
    }

    if is_coinbase {
      for flotsam in inscriptions {
        let new_satpoint = SatPoint {
          outpoint: OutPoint::null(),
          offset: self.lost_sats + flotsam.offset - output_value,
        };
        self.update_inscription_location(input_sat_ranges, flotsam, new_satpoint)?;
      }
      self.lost_sats += self.reward - output_value;
      Ok(())
    } else {
      self.flotsam.extend(inscriptions.map(|flotsam| Flotsam {
        offset: self.reward + flotsam.offset - output_value,
        ..flotsam
      }));
      self.reward += input_value - output_value;
      Ok(())
    }
  }

  fn calculate_sat(
    input_sat_ranges: Option<&VecDeque<(u64, u64)>>,
    input_offset: u64,
  ) -> Option<Sat> {
    let mut sat = None;
    if let Some(input_sat_ranges) = input_sat_ranges {
      let mut offset = 0;
      for (start, end) in input_sat_ranges {
        let size = end - start;
        if offset + size > input_offset {
          let n = start + input_offset - offset;
          sat = Some(Sat(n));
          break;
        }
        offset += size;
      }
    }
    sat
  }

  fn update_inscription_location(
    &mut self,
    input_sat_ranges: Option<&VecDeque<(u128, u128)>>,
    flotsam: Flotsam,
    new_satpoint: SatPoint,
  ) -> Result {
    let inscription_id = flotsam.inscription_id.store();
    let unbound = match flotsam.origin {
      Origin::Old { old_satpoint } => {
        self.satpoint_to_id.remove_all(&old_satpoint.store())?;

        false
      }
      Origin::New {
        fee,
        cursed,
        unbound,
      } => {
        let number = if cursed {
          let next_cursed_number = self.next_cursed_number;
          self.next_cursed_number -= 1;

<<<<<<< HEAD
        let mut sat = None;
        if let Some(input_sat_ranges) = input_sat_ranges {
          let mut offset = 0;
          for (start, end) in input_sat_ranges {
            let size = end - start;
            if offset + size > flotsam.offset as u128 {
              let n = start + flotsam.offset as u128 - offset;
              self.sat_to_inscription_id.insert(&n, &inscription_id)?;
              sat = Some(Sat(n));
              break;
=======
          next_cursed_number
        } else {
          let next_number = self.next_number;
          self.next_number += 1;

          next_number
        };

        self.number_to_id.insert(number, &inscription_id)?;

        let sat = if unbound {
          None
        } else {
          let mut sat = None;
          if let Some(input_sat_ranges) = input_sat_ranges {
            let mut offset = 0;
            for (start, end) in input_sat_ranges {
              let size = end - start;
              if offset + size > flotsam.offset {
                let n = start + flotsam.offset - offset;
                self.sat_to_inscription_id.insert(&n, &inscription_id)?;
                sat = Some(Sat(n));
                break;
              }
              offset += size;
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
            }
          }
          sat
        };

        self.id_to_entry.insert(
          &inscription_id,
          &InscriptionEntry {
            fee,
            height: self.height,
            number,
            sat,
            timestamp: self.timestamp,
          }
          .store(),
        )?;

        unbound
      }
    };

    let satpoint = if unbound {
      let new_unbound_satpoint = SatPoint {
        outpoint: unbound_outpoint(),
        offset: self.unbound_inscriptions,
      };
      self.unbound_inscriptions += 1;
      new_unbound_satpoint.store()
    } else {
      new_satpoint.store()
    };

    self.satpoint_to_id.insert(&satpoint, &inscription_id)?;
    self.id_to_satpoint.insert(&inscription_id, &satpoint)?;

    Ok(())
  }
}
