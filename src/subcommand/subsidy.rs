use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Subsidy {
  #[clap(help = "List sats in subsidy at <HEIGHT>.")]
  height: Height,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub first: u128,
  pub subsidy: u64,
}

impl Subsidy {
  pub(crate) fn run(self) -> SubcommandResult {
    let first = self.height.starting_sat();

    let subsidy = self.height.subsidy();

    if subsidy == 0 {
      bail!("block {} has no subsidy", self.height);
    }

    Ok(Box::new(Output {
      first: first.0,
      subsidy,
<<<<<<< HEAD
    })?;

    Ok(())
=======
      name: first.name(),
    }))
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
  }
}
