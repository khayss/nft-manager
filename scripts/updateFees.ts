import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  let txSig = await nftManagerProgram.methods
    .updateFees({
      fee: { sellFee: {} },
      newFee: 250,
    })
    .rpc();

  console.log(`Transaction: ${txSig}`);
}

main().catch(console.error);
