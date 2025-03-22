import * as anchor from "@coral-xyz/anchor";

import { goldPriceFeed, nftManagerProgram, solPriceFeed } from ".";

async function main() {
  const buffer64 = Buffer.alloc(8);
  buffer64.writeBigInt64LE(BigInt(1), 0);
  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[11].value)),
      buffer64,
    ],
    nftManagerProgram.programId
  );
  try {
    const tx = await nftManagerProgram.methods
      .burnNft(new anchor.BN(1))
      .accountsPartial({
        mint: mintPda,
      })
      .rpc();
    console.log("Transaction signature:", tx);
  } catch (error) {
    console.error("Error:", error);
  }
}

main().catch(console.error);
