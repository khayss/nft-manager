import { web3 } from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  try {
    const createColIx = await nftManagerProgram.methods
      .createCollection({
        name: "JPG NFT Collection",
        symbol: "JPG",
        uri: "https://arweave.net/123",
      })
      .instruction();
    const initIx = await nftManagerProgram.methods
      .initializeNftManager({
        fractionalizeFee: 150,
        sellFee: 250,
      })
      .instruction();

    const initTx = new web3.Transaction().add(createColIx).add(initIx);

    const initTxSig = await nftManagerProgram.provider.sendAndConfirm(initTx);

    console.log("Transaction signature:", initTxSig);
  } catch (error) {
    console.error("Error:", error);
  }
}

main().catch(console.error);
