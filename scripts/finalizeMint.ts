import * as anchor from "@coral-xyz/anchor";

import { nftManagerProgram } from ".";

async function main() {
  try {
    const tx = await nftManagerProgram.methods
      .finalizeMintNft(new anchor.BN(3))
      .rpc();
    console.log("Transaction signature:", tx);
  } catch (error) {
    console.error("Error:", error);
  }
}

main().catch(console.error);
