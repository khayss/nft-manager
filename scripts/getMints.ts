import * as anchor from "@coral-xyz/anchor";

import { nftManagerProgram } from ".";

async function main() {
  let [nftManagerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[12].value))],
    nftManagerProgram.programId
  );

  let nftManagerData = await nftManagerProgram.account.nftManager.fetch(
    nftManagerPda
  );

  const totalMints = new anchor.BN(10); // Example BN value
  const totalMintsNumber = totalMints.toNumber(); // Convert BN to number

  for (let i = 0; i < nftManagerData.discriminant.toNumber(); i++) {
    let [mint] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[11].value)),
        new anchor.BN(i).toBuffer("le"),
      ],
      nftManagerProgram.programId
    );

    console.log(`Mint ${i}: ${mint.toBase58()}`);
  }
}

main().catch(console.error);
