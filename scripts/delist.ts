import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  const discriminant = new anchor.BN(12);

  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[11].value)),
      discriminant.toArrayLike(Buffer, "le", 8),
    ],
    nftManagerProgram.programId
  );
  let txSig = await nftManagerProgram.methods
    .delistNft(discriminant)
    .accountsPartial({ mint: mintPda })
    .rpc();

  console.log(`Transaction: ${txSig}`);
}

main().catch(console.error);
