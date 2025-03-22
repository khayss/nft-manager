import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  const buffer64 = Buffer.alloc(8);
  buffer64.writeBigInt64LE(BigInt(0), 0);

  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[11].value)),
      buffer64,
    ],
    nftManagerProgram.programId
  );
  let txSig = await nftManagerProgram.methods
    .listNft(
      new anchor.BN(0),
      new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL)
    )
    .accountsPartial({ mint: mintPda })
    .rpc();

  console.log(`Transaction: ${txSig}`);
}

main().catch(console.error);
