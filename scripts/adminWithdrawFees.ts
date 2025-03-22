import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  const [feesCollectorPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[4].value))],
    nftManagerProgram.programId
  );
  const data = await nftManagerProgram.provider.connection.getAccountInfo(
    feesCollectorPda
  );
  const balance = data.lamports;
  const rentExempt =
    await nftManagerProgram.provider.connection.getMinimumBalanceForRentExemption(
      data.data.length
    );

  let txSig = await nftManagerProgram.methods
    .adminWithdrawFees(new anchor.BN(balance / 10 - rentExempt))
    .accounts({ recipient: nftManagerProgram.provider.publicKey })
    .rpc();

  console.log(`Transaction: ${txSig}`);
}

main().catch(console.error);
