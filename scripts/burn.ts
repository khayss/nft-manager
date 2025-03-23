import * as anchor from "@coral-xyz/anchor";

import { goldPriceFeed, nftManagerProgram, solPriceFeed } from ".";

async function main() {
  const discriminant = new anchor.BN(12);
  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[11].value)),
      discriminant.toArrayLike(Buffer, "le", 8),
    ],
    nftManagerProgram.programId
  );
  try {
    const tx = await nftManagerProgram.methods
      .burnNft(discriminant)
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
