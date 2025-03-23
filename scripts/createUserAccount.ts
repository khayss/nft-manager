import { nftManagerProgram } from ".";

async function main() {
  let txSig = await nftManagerProgram.methods.createUserAccount().rpc();

  console.log(`Transaction: ${txSig}`);
}

main().catch(console.error);
