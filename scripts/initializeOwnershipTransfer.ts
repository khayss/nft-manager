import { nftManagerProgram, wallet } from ".";

async function main() {
  let txSig = await nftManagerProgram.methods
    .initiailizeOwnershipTransfer()
    .accounts({ newOwner: wallet.publicKey })
    .rpc();

  console.log(`Transaction: ${txSig}`);
}

main().catch(console.error);
