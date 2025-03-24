import * as anchor from "@coral-xyz/anchor";
import { IdlEvents } from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  type EventType = IdlEvents<typeof nftManagerProgram.idl>;
  type BuyNftEvent = EventType["buyNftEvent"];
  const eventParser = new anchor.EventParser(
    nftManagerProgram.programId,
    new anchor.BorshCoder(nftManagerProgram.idl)
  );

  const txSig =
    "47muFAkChAGsgybRspXAukda41bRwUdSTPE9zinZ2wZR4RuGyyhG5WgX9NksD1EssBmQon6wfQEVDTB5VMev9DfC";

  const tx = await nftManagerProgram.provider.connection.getTransaction(txSig, {
    commitment: "confirmed",
    maxSupportedTransactionVersion: 2,
  });

  const events = eventParser.parseLogs(tx.meta.logMessages || []);

  for (let event of events) {
    if (event.name === "buyNftEvent") {
      const buyNftEvent = event.data as BuyNftEvent;
      console.log("Mint: ", buyNftEvent.mint.toBase58());
      console.log("Seller: ", buyNftEvent.seller.toBase58());
      console.log("Buyer: ", buyNftEvent.buyer.toBase58());
      console.log("Price: ", buyNftEvent.price.toString());
    }
  }
}

main().catch(console.error);
