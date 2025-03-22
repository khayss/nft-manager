import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  const [userAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[13].value)),
      nftManagerProgram.provider.publicKey.toBuffer(),
    ],
    nftManagerProgram.programId
  );
  const data = await nftManagerProgram.provider.connection.getAccountInfo(
    userAccountPda
  );
  const balance = data.lamports;
  const rentExempt =
    await nftManagerProgram.provider.connection.getMinimumBalanceForRentExemption(
      data.data.length
    );

  let txSig = await nftManagerProgram.methods
    .userWithdraw(new anchor.BN(balance / 10 - rentExempt))
    .rpc();

  console.log(`Transaction: ${txSig}`);
}

main().catch(console.error);
