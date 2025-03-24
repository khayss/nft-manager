import * as anchor from "@coral-xyz/anchor";
import { IdlEvents } from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  type EventType = IdlEvents<typeof nftManagerProgram.idl>;
  type BuyNftEvent = EventType["mintNftEvent"];
  const eventParser = new anchor.EventParser(
    nftManagerProgram.programId,
    new anchor.BorshCoder(nftManagerProgram.idl)
  );

  const txSig =
    "21nBFQD99GxNAs2KnMJUNjJrwt2V1Kp9oVS5ZeJG13cL2f5umyuHGz6iEYsphhP2xP4STDz5XGyiVVC6aow5KM2o";

  const tx = await nftManagerProgram.provider.connection.getTransaction(txSig, {
    commitment: "confirmed",
    maxSupportedTransactionVersion: 2,
  });

  const events = eventParser.parseLogs(tx.meta.logMessages || []);

  for (let event of events) {
    if (event.name === "mintNftEvent") {
      const buyNftEvent = event.data as BuyNftEvent;
      console.log("Mint: ", buyNftEvent.mint.toBase58());
      console.log("Seller: ", buyNftEvent.finalizeData.toBase58());
      console.log("Buyer: ", buyNftEvent.recipient.toBase58());
      console.log("Price: ", buyNftEvent.price.toString());

      const tx = await nftManagerProgram.methods
        .finalizeMintNft(buyNftEvent.discriminant)
        .rpc();
      console.log("Finalize transaction signature:", tx);
    }
  }
}

main().catch(console.error);
