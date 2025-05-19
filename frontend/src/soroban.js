import {
  Server,
  Networks,
  Keypair,
  TransactionBuilder,
  Operation,
  Asset,
} from "@stellar/soroban-client";

export async function invoke(contractId, fn, args = {}) {
  const server = new Server("https://horizon-testnet.stellar.org");
  const networkPassphrase = Networks.TESTNET;
  // create a quick Friendbot-funded keypair
  const pair = Keypair.random();
  await server.friendbot(pair.publicKey());
  let account = await server.loadAccount(pair.publicKey());
  let tx = new TransactionBuilder(account, {
    fee: "100",
    networkPassphrase,
  })
    .addOperation(
      Operation.invokeHostFunction({
        function: fn,
        args: Object.entries(args).map(([k, v]) => ({ [k]: v })),
      })
    )
    .setTimeout(30)
    .build();
  tx.sign(pair);
  return server.submitTransaction(tx);
}
