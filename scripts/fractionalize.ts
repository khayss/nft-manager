import * as anchor from "@coral-xyz/anchor";

import { goldPriceFeed, nftManagerProgram, solPriceFeed } from ".";

async function main() {
  const WEIGHT = new anchor.BN(10);

  const currentMintDiscriminant = new anchor.BN(1);
  const partAWeight = new anchor.BN(3);
  const partBWeight = new anchor.BN(3);

  const mintEventId = nftManagerProgram.addEventListener(
    "fractionalizeNftEvent",
    (event) => {
      console.log("Event received: ", event);
    }
  );

  try {
    const fractionalizeNftIx = await nftManagerProgram.methods
      .fractionalizeNft({
        discriminant: currentMintDiscriminant,
        partA: {
          name: "Part A",
          symbol: "PA",
          uri: "https://parta.com",
          weight: partAWeight,
        },
        partB: {
          name: "Part B",
          symbol: "PB",
          uri: "https://partb.com",
          weight: partBWeight,
        },
      })
      .accounts({
        goldPriceUpdate: goldPriceFeed,
        solPriceUpdate: solPriceFeed,
      })
      .instruction();

    const finalizeFractionalizeNftIx = await nftManagerProgram.methods
      .finalizeFractionalizeNft(currentMintDiscriminant)
      .instruction();

    const tx = new anchor.web3.Transaction()
      .add(fractionalizeNftIx)
      .add(finalizeFractionalizeNftIx);

    const txResult = await nftManagerProgram.provider.sendAndConfirm(tx);

    console.log("Transaction signature:", txResult);
  } catch (error) {
    console.error("Error:", error);
  }

  await nftManagerProgram.removeEventListener(mintEventId);
}

main().catch(console.error);
