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

  const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[7].value)),
      mintPda.toBuffer(),
      nftManagerProgram.provider.publicKey.toBuffer(),
    ],
    nftManagerProgram.programId
  );

  const data = await nftManagerProgram.account.listing.fetch(listingPda);
  console.log(data);
  console.log(data.price.toString());

  let txSig = await nftManagerProgram.methods
    .updateListingPrice({
      discriminant: new anchor.BN(0),
      newPrice: new anchor.BN(0.2 * anchor.web3.LAMPORTS_PER_SOL),
    })
    // .accountsPartial({ mint: mintPda })
    .rpc();

  console.log(`Transaction: ${txSig}`);

  const dataAfter = await nftManagerProgram.account.listing.fetch(listingPda);
  console.log(dataAfter);
  console.log(dataAfter.price.toString());
}

main().catch(console.error);
