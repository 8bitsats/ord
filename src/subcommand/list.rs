use super::*;

#[derive(Debug, Parser)]
pub(crate) struct List {
  #[clap(help = "List sats in <OUTPOINT>.")]
  outpoint: OutPoint,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub output: OutPoint,
<<<<<<< HEAD
  pub start: u128,
=======
  pub start: u64,
  pub end: u64,
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
  pub size: u64,
  pub offset: u64,
  pub rarity: Rarity,
}

impl List {
  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let index = Index::open(&options)?;

    index.update()?;

    match index.list(self.outpoint)? {
      Some(crate::index::List::Unspent(ranges)) => {
        let mut outputs = Vec::new();
<<<<<<< HEAD
        for (output, start, size, rarity) in list(self.outpoint, ranges) {
=======
        for Output {
          output,
          start,
          end,
          size,
          offset,
          rarity,
          name,
        } in list(self.outpoint, ranges)
        {
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
          outputs.push(Output {
            output,
            start,
            end,
            size,
            offset,
            rarity,
          });
        }

        Ok(Box::new(outputs))
      }
      Some(crate::index::List::Spent) => Err(anyhow!("output spent.")),
      None => Err(anyhow!("output not found")),
    }
  }
}

<<<<<<< HEAD
fn list(outpoint: OutPoint, ranges: Vec<(u128, u128)>) -> Vec<(OutPoint, u128, u64, Rarity)> {
  ranges
    .into_iter()
    .map(|(start, end)| {
      let size = u64::try_from(end - start).unwrap();
      let rarity = Sat(start).rarity();

      (outpoint, start, size, rarity)
=======
fn list(outpoint: OutPoint, ranges: Vec<(u64, u64)>) -> Vec<Output> {
  let mut offset = 0;
  ranges
    .into_iter()
    .map(|(start, end)| {
      let size = end - start;
      let output = Output {
        output: outpoint,
        start,
        end,
        size,
        offset,
        name: Sat(start).name(),
        rarity: Sat(start).rarity(),
      };

      offset += size;

      output
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn output(
    output: OutPoint,
    start: u64,
    end: u64,
    size: u64,
    offset: u64,
    rarity: Rarity,
    name: String,
  ) -> super::Output {
    super::Output {
      output,
      start,
      end,
      size,
      offset,
      name,
      rarity,
    }
  }

  #[test]
  #[ignore]
  fn list_ranges() {
    let outpoint =
      OutPoint::from_str("1a91e3dace36e2be3bf030a65679fe821aa1d6ef92e7c9902eb318182c355691:5")
        .unwrap();
    let ranges = vec![
      (50 * COIN_VALUE as u128, 55 * COIN_VALUE as u128),
      (10 as u128, 100 as u128),
      (1050000000000000 as u128, 1150000000000000 as u128),
    ];
    assert_eq!(
      list(outpoint, ranges),
      vec![
<<<<<<< HEAD
        (
          OutPoint::from_str("1a91e3dace36e2be3bf030a65679fe821aa1d6ef92e7c9902eb318182c355691:5")
            .unwrap(),
          50 * COIN_VALUE as u128,
=======
        output(
          OutPoint::from_str("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:5")
            .unwrap(),
          50 * COIN_VALUE,
          55 * COIN_VALUE,
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
          5 * COIN_VALUE,
          0,
          Rarity::Uncommon,
        ),
<<<<<<< HEAD
        (
          OutPoint::from_str("1a91e3dace36e2be3bf030a65679fe821aa1d6ef92e7c9902eb318182c355691:5")
=======
        output(
          OutPoint::from_str("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:5")
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
            .unwrap(),
          10,
          100,
          90,
          5 * COIN_VALUE,
          Rarity::Common,
        ),
<<<<<<< HEAD
        (
          OutPoint::from_str("1a91e3dace36e2be3bf030a65679fe821aa1d6ef92e7c9902eb318182c355691:5")
=======
        output(
          OutPoint::from_str("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:5")
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
            .unwrap(),
          1050000000000000,
          1150000000000000,
          100000000000000,
          5 * COIN_VALUE + 90,
          Rarity::Epic,
        )
      ]
    )
  }
}
