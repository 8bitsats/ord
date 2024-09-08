use {super::*, ord::subcommand::list::Output};

#[test]
fn output_found() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  let output = CommandBuilder::new(
    "--index-sats list 5b2a3f53f605d62c53e62932dac6925e3d74afa5a4b459745c36d42d0ed26a69:0",
  )
  .rpc_server(&rpc_server)
  .run_and_deserialize_output::<Vec<Output>>();

  assert_eq!(
    output,
    vec![Output {
      output: "5b2a3f53f605d62c53e62932dac6925e3d74afa5a4b459745c36d42d0ed26a69:0"
        .parse()
        .unwrap(),
      start: 0,
<<<<<<< HEAD
      size: 8800000000,
=======
      end: 50 * COIN_VALUE,
      size: 50 * COIN_VALUE,
      offset: 0,
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
      rarity: "mythic".parse().unwrap(),
    }]
  );
}

#[test]
fn output_not_found() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  CommandBuilder::new(
    "--index-sats list 0000000000000000000000000000000000000000000000000000000000000000:0",
  )
  .rpc_server(&rpc_server)
  .expected_exit_code(1)
  .expected_stderr("error: output not found\n")
  .run_and_extract_stdout();
}

#[test]
fn no_satoshi_index() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  CommandBuilder::new("list 1a91e3dace36e2be3bf030a65679fe821aa1d6ef92e7c9902eb318182c355691:0")
    .rpc_server(&rpc_server)
    .expected_stderr("error: list requires index created with `--index-sats` flag\n")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}
