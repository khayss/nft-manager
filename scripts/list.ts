import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  const discriminant = new anchor.BN(14);

  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[12].value)),
      discriminant.toArrayLike(Buffer, "le", 8),
    ],
    nftManagerProgram.programId
  );

  console.log(`Mint PDA: ${mintPda.toBase58()}`);

  let txSig = await nftManagerProgram.methods
    .listNft({ discriminant, price: new anchor.BN(10_000) })
    .accountsPartial({ mint: mintPda })
    .rpc();

  console.log(`Transaction: ${txSig}`);

  const accounts = await nftManagerProgram.account.listing.all();
  for (let account of accounts) {
    console.log(account.publicKey.toBase58());
    console.log("Mint: ", account.account.mint.toBase58());
    console.log("Owner: ", account.account.owner.toBase58());
  }
}

main().catch(console.error);
