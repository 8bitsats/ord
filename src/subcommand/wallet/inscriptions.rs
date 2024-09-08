use {super::*, crate::wallet::Wallet};

#[derive(Serialize, Deserialize)]
pub struct Output {
  pub inscription: InscriptionId,
  pub location: SatPoint,
  pub explorer: String,
  pub postage: u64,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;
  index.update()?;

  let unspent_outputs = index.get_unspent_outputs(Wallet::load(&options)?)?;
  let inscriptions = index.get_inscriptions(unspent_outputs.clone())?;

  let explorer = match options.chain() {
    Chain::Mainnet => "https://ordinals.com/shibescription/",
    Chain::Regtest => "http://localhost/shibescription/",
    Chain::Signet => "https://signet.ordinals.com/shibescription/",
    Chain::Testnet => "https://testnet.ordinals.com/shibescription/",
  };

  let mut output = Vec::new();

  for (location, inscription) in inscriptions {
    if let Some(postage) = unspent_outputs.get(&location.outpoint) {
      output.push(Output {
        location,
        inscription,
        explorer: format!("{explorer}{inscription}"),
        postage: postage.to_sat(),
      })
    }
  }

  Ok(Box::new(output))
}
