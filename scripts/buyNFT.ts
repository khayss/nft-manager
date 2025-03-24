import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram, solPriceFeed, wallet } from ".";

async function main() {
  const buyNftEventId = nftManagerProgram.addEventListener(
    "buyNftEvent",
    (event) => {
      console.log("Mint: ", event.mint.toBase58());
      console.log("Seller: ", event.seller.toBase58());
      console.log("Buyer: ", event.buyer.toBase58());
      console.log("Recipient: ", event.recipient.toBase58());
      console.log("Price: ", event.price.toString());
    }
  );
  const discriminant = new anchor.BN(17);

  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[12].value)),
      discriminant.toArrayLike(Buffer, "le", 8),
    ],
    nftManagerProgram.programId
  );

  const listingAccounts = await nftManagerProgram.account.listing.all([
    {
      memcmp: {
        offset: 8 + 8 + 32,
        bytes: mintPda.toBase58(),
      },
    },
  ]);

  const listing = listingAccounts[0];

  //   await nftManagerProgram.methods.createUserAccount().rpc();

  let txSig = await nftManagerProgram.methods
    .buyNft(discriminant)
    .accountsPartial({
      mint: mintPda,
      seller: listing.account.owner,
      solPriceUpdate: solPriceFeed,
      recipient: wallet.publicKey,
    })
    .rpc();

  console.log(`Transaction: ${txSig}`);

  await nftManagerProgram.removeEventListener(buyNftEventId);
}

main().catch(console.error);
