import { nftManagerProgram } from ".";

async function main() {
  let txSig = await nftManagerProgram.methods.finalizeOwnershipTransfer().rpc();

  console.log(`Transaction: ${txSig}`);
}

main().catch(console.error);
