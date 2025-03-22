import * as anchor from "@coral-xyz/anchor";
import { nftManagerProgram } from ".";

async function main() {
  const [nftManagerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[12].value))],
    nftManagerProgram.programId
  );
  const [mintFeesCollectorPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Uint8Array.from(JSON.parse(nftManagerProgram.idl.constants[10].value))],
    nftManagerProgram.programId
  );
  const data = await nftManagerProgram.provider.connection.getAccountInfo(
    mintFeesCollectorPda
  );
  const balance = data.lamports;
  const rentExempt =
    await nftManagerProgram.provider.connection.getMinimumBalanceForRentExemption(
      data.data.length
    );

  let txSig = await nftManagerProgram.methods
    .adminWithdrawMintFees(new anchor.BN(balance / 100 - rentExempt))
    .accounts({ recipient: nftManagerPda })
    .rpc();

  console.log(`Transaction: ${txSig}`);
}

main().catch(console.error);
