import * as anchor from "@coral-xyz/anchor";

import { goldPriceFeed, nftManagerProgram, solPriceFeed } from ".";

async function main() {
  const NAME = "100g Gold Bar";
  const SYMBOL = "GOLD";
  const URI = "https://arweave.net/123";
  const WEIGHT = new anchor.BN(10);

  const mintEventId = nftManagerProgram.addEventListener(
    "mintNftEvent",
    (event) => {
      console.log("Mint: ", event.mint.toString());
      console.log("Discriminant: ", event.discriminant.toString());
      console.log(
        "Price in Sol: ",
        event.price.toNumber() / anchor.web3.LAMPORTS_PER_SOL
      );
      nftManagerProgram.methods
        .finalizeMintNft(event.discriminant)
        .rpc()
        .then((finalizeTx) => {
          console.log("Finalize transaction signature:", finalizeTx);
        })
        .catch(console.error);
    }
  );

  try {
    const tx = await nftManagerProgram.methods
      .mintNft({ name: NAME, symbol: SYMBOL, uri: URI, weight: WEIGHT })
      .accounts({
        goldPriceUpdate: goldPriceFeed,
        solPriceUpdate: solPriceFeed,
      })
      .rpc();
    console.log("Transaction signature:", tx);
  } catch (error) {
    console.error("Error:", error);
  }

  await nftManagerProgram.removeEventListener(mintEventId);
}

main().catch(console.error);
