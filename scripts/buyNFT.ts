import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram, solPriceFeed, wallet } from ".";

async function main() {
  const buyNftEventId = nftManagerProgram.addEventListener(
    "buyNftEvent",
    (event) => {
      console.log("Mint: ", event.mint.toBase58());
      console.log("Seller: ", event.seller.toBase58());
      console.log("Buyer: ", event.buyer.toBase58());
      console.log("Price: ", event.price.toString());
    }
  );
  const discriminant = new anchor.BN(14);

  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[12].value)),
      discriminant.toArrayLike(Buffer, "le", 8),
    ],
    nftManagerProgram.programId
  );
  const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[7].value)),
      mintPda.toBuffer(),
      wallet.publicKey.toBuffer(),
    ],
    nftManagerProgram.programId
  );

  const listingData = await nftManagerProgram.account.listing.fetch(listingPda);

  //   await nftManagerProgram.methods.createUserAccount().rpc();

  let txSig = await nftManagerProgram.methods
    .buyNft(discriminant)
    .accountsPartial({
      mint: mintPda,
      seller: listingData.owner,
      solPriceUpdate: solPriceFeed,
    })
    .rpc();

  console.log(`Transaction: ${txSig}`);

  await nftManagerProgram.removeEventListener(buyNftEventId);
}

main().catch(console.error);
