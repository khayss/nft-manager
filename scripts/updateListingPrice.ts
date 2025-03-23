import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  const listingEventId = nftManagerProgram.addEventListener(
    "listNftEvent",
    (event) => {
      console.log("Listing: ", event.listing.toString());
      console.log("Listing: ", event.listing.toString());
      console.log(
        "Price in Sol: ",
        event.price.toNumber() / anchor.web3.LAMPORTS_PER_SOL
      );
    }
  );

  const discriminant = new anchor.BN(11);

  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[11].value)),
      discriminant.toArrayLike(Buffer, "le", 8),
    ],
    nftManagerProgram.programId
  );

  const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[7].value)),
      mintPda.toBuffer(),
    ],
    nftManagerProgram.programId
  );

  let txSig = await nftManagerProgram.methods
    .updateListingPrice({
      discriminant,
      newPrice: new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL),
    })
    .rpc();

  console.log(`Transaction: ${txSig}`);

  await nftManagerProgram.removeEventListener(listingEventId);
}

main().catch(console.error);
