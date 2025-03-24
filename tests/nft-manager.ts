import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftManager } from "../target/types/nft_manager";
import { expect } from "chai";
import {
  collectionName,
  collectionSymbol,
  collectionUri,
  fractionalizeFee,
  getPda,
  goldPriceUpdateKey,
  createMintMetadata,
  Pda,
  sellFee,
  solPriceUpdateKey,
  getAdditionMetadata,
  Metadata,
} from "./utils";
import {
  getMint,
  getTokenMetadata,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

describe("nft-manager", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.NftManager as Program<NftManager>;

  const [nftManagerPda, nftMangerPdaBump] = getPda(program, Pda.NftManager);
  const [collectionPda, collectionPdaBump] = getPda(program, Pda.Collection);
  const [feesCollectorPda, feesCollectorPdaBump] = getPda(
    program,
    Pda.FeesCollector
  );
  const [mintFeesCollectorPda, mintFeesCollectorPdaBump] = getPda(
    program,
    Pda.MintFeesCollector
  );

  const newAuthority = anchor.web3.Keypair.generate();

  before(async () => {
    // Add your test here.
    const createCollectionIx = await program.methods
      .createCollection({
        name: collectionName,
        symbol: collectionSymbol,
        uri: collectionUri,
      })
      .instruction();

    const initializeNftManagerIx = await program.methods
      .initializeNftManager({
        fractionalizeFee,
        sellFee,
      })
      .instruction();

    const tx = new anchor.web3.Transaction()
      .add(createCollectionIx)
      .add(initializeNftManagerIx);

    await program.provider.sendAndConfirm(tx);

    // Airdrop SOL to new Authority
    const signature = await program.provider.connection.requestAirdrop(
      newAuthority.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );

    const { blockhash, lastValidBlockHeight } =
      await program.provider.connection.getLatestBlockhash();
    await program.provider.connection.confirmTransaction({
      signature,
      blockhash,
      lastValidBlockHeight,
    });
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const nftManagerData = await program.account.nftManager.fetch(
      nftManagerPda
    );
    const feesCollectorData = await program.account.feesCollector.fetch(
      feesCollectorPda
    );
    const mintFeesCollectorData = await program.account.mintFeesCollector.fetch(
      mintFeesCollectorPda
    );

    // NFT Manager Data
    expect(nftManagerData.authority.toBase58()).equals(
      program.provider.publicKey.toBase58(),
      "Authority is not correct"
    );
    expect(nftManagerData.bump).equals(nftMangerPdaBump, "Bump is not correct");
    expect(nftManagerData.collection.toBase58()).equals(
      collectionPda.toBase58(),
      "Collection is not correct"
    );
    expect(nftManagerData.discriminant.toString()).equals(
      "0",
      "Discriminant is not correct"
    );
    expect(nftManagerData.futureAuthority).equals(null),
      "Future Authority is not correct";

    // Fees Collector Data
    expect(feesCollectorData.bump).equals(
      feesCollectorPdaBump,
      "Bump is not correct"
    );
    expect(feesCollectorData.sellFee).equals(
      sellFee,
      "Sell Fee is not correct"
    );
    expect(feesCollectorData.fractionalizeFee).equals(
      fractionalizeFee,
      "Fractionalize Fee is not correct"
    );
    expect(feesCollectorData.feesDecimals).equals(
      4,
      "Fees Decimals is not correct"
    );

    // Mint Fees Collector Data
    expect(mintFeesCollectorData.bump).equals(
      mintFeesCollectorPdaBump,
      "Bump is not correct"
    );
  });

  it("Mint NFT", async () => {
    const weight = new anchor.BN(10);
    const nftManagerData = await program.account.nftManager.fetch(
      nftManagerPda
    );
    const [mintPda, mintPdaBump] = getPda(program, Pda.Mint, [
      nftManagerData.discriminant.toArrayLike(Buffer, "le", 8),
    ]);

    const mintNftIx = await program.methods
      .mintNft({
        name: createMintMetadata.name,
        symbol: createMintMetadata.symbol,
        uri: createMintMetadata.uri,
        weight,
      })
      .accounts({
        goldPriceUpdate: goldPriceUpdateKey,
        solPriceUpdate: solPriceUpdateKey,
        recipient: program.provider.publicKey,
      })
      .instruction();

    const finalizeMintNftIx = await program.methods
      .finalizeMintNft(nftManagerData.discriminant)
      .instruction();
    const tx = new anchor.web3.Transaction()
      .add(mintNftIx)
      .add(finalizeMintNftIx);
    await program.provider.sendAndConfirm(tx);

    const nftManagerDataAfter = await program.account.nftManager.fetch(
      nftManagerPda
    );
    const mint = await getMint(
      program.provider.connection,
      mintPda,
      "processed",
      TOKEN_2022_PROGRAM_ID
    );
    const mintMetadata = await getTokenMetadata(
      program.provider.connection,
      mintPda,
      "processed",
      TOKEN_2022_PROGRAM_ID
    );
    const metadataWeight = getAdditionMetadata(
      Metadata.Weight,
      mintMetadata.additionalMetadata
    );
    const metadataDiscriminant = getAdditionMetadata(
      Metadata.Discriminant,
      mintMetadata.additionalMetadata
    );
    const metadataCollection = getAdditionMetadata(
      Metadata.Collection,
      mintMetadata.additionalMetadata
    );

    expect(nftManagerDataAfter.discriminant.eq(new anchor.BN(1))).to.equal(
      true,
      "Mint Count is not correct"
    );
    expect(mint.decimals).to.equal(0, "Decimals is not correct");
    expect(mint.supply).to.equal(BigInt(1), "Supply is not correct");
    expect(mint.isInitialized).to.equal(true, "Mint Authority is not correct");
    expect(mintMetadata.name).to.equal(
      createMintMetadata.name,
      "Name is not correct"
    );
    expect(mintMetadata.symbol).to.equal(
      createMintMetadata.symbol,
      "Symbol is not correct"
    );
    expect(mintMetadata.uri).to.equal(
      createMintMetadata.uri,
      "Uri is not correct"
    );
    expect(mintMetadata.updateAuthority.toBase58()).to.equal(
      mintPda.toBase58(),
      "Update Authority is not correct"
    );
    expect(metadataWeight).to.equal(weight.toString(), "Weight is not correct");
    expect(metadataDiscriminant).to.equal(
      nftManagerData.discriminant.toString(),
      "Discriminant is not correct"
    );
    expect(metadataCollection).to.equal(
      collectionPda.toBase58(),
      "Collection is not correct"
    );
  });

  it("Fractionalize NFT", async () => {
    // const weight = new anchor.BN(10);
    const partAWeight = new anchor.BN(5);
    const partBWeight = new anchor.BN(5);
    const mintDiscriminant = new anchor.BN(0);
    const mint2Discriminant = new anchor.BN(1);

    const [mintPda, mintPdaBump] = getPda(program, Pda.Mint, [
      mintDiscriminant.toArrayLike(Buffer, "le", 8),
    ]);

    const fractionalizeNftIx = await program.methods
      .fractionalizeNft({
        discriminant: mintDiscriminant,
        partA: {
          name: createMintMetadata.name,
          symbol: createMintMetadata.symbol,
          uri: createMintMetadata.uri,
          weight: partAWeight,
        },
        partB: {
          name: createMintMetadata.name,
          symbol: createMintMetadata.symbol,
          uri: createMintMetadata.uri,
          weight: partBWeight,
        },
      })
      .accounts({
        goldPriceUpdate: goldPriceUpdateKey,
        solPriceUpdate: solPriceUpdateKey,
      })
      .instruction();
    const finalizeFractionalizeNftIx = await program.methods
      .finalizeFractionalizeNft(mintDiscriminant)
      .instruction();
    const tx2 = new anchor.web3.Transaction()
      .add(fractionalizeNftIx)
      .add(finalizeFractionalizeNftIx);
    await program.provider.sendAndConfirm(tx2);

    const [mint2Pda, mint2PdaBump] = getPda(program, Pda.Mint, [
      mint2Discriminant.toArrayLike(Buffer, "le", 8),
    ]);

    const nftManagerDataAfter = await program.account.nftManager.fetch(
      nftManagerPda
    );
    const mint = await getMint(
      program.provider.connection,
      mintPda,
      "processed",
      TOKEN_2022_PROGRAM_ID
    );
    const mint2 = await getMint(
      program.provider.connection,
      mint2Pda,
      "processed",
      TOKEN_2022_PROGRAM_ID
    );
    const mintMetadata = await getTokenMetadata(
      program.provider.connection,
      mintPda,
      "processed",
      TOKEN_2022_PROGRAM_ID
    );
    const mint2Metadata = await getTokenMetadata(
      program.provider.connection,
      mint2Pda,
      "processed",
      TOKEN_2022_PROGRAM_ID
    );
    const metadataWeight = getAdditionMetadata(
      Metadata.Weight,
      mintMetadata.additionalMetadata
    );
    const metadataWeight2 = getAdditionMetadata(
      Metadata.Weight,
      mint2Metadata.additionalMetadata
    );
    const metadataDiscriminant = getAdditionMetadata(
      Metadata.Discriminant,
      mintMetadata.additionalMetadata
    );
    const metadataDiscriminant2 = getAdditionMetadata(
      Metadata.Discriminant,
      mint2Metadata.additionalMetadata
    );
    const metadataCollection = getAdditionMetadata(
      Metadata.Collection,
      mintMetadata.additionalMetadata
    );
    const metadataCollection2 = getAdditionMetadata(
      Metadata.Collection,
      mint2Metadata.additionalMetadata
    );

    expect(nftManagerDataAfter.discriminant.eq(new anchor.BN(2))).to.equal(
      true,
      "Mint Count is not correct"
    );

    expect(mint.decimals).to.equal(0, "Decimals is not correct");
    expect(mint.supply).to.equal(BigInt(1), "Supply is not correct");
    expect(mint.isInitialized).to.equal(true, "Mint Authority is not correct");
    expect(mintMetadata.name).to.equal(
      createMintMetadata.name,
      "Name is not correct"
    );
    expect(mintMetadata.symbol).to.equal(
      createMintMetadata.symbol,
      "Symbol is not correct"
    );
    expect(mintMetadata.uri).to.equal(
      createMintMetadata.uri,
      "Uri is not correct"
    );
    expect(metadataWeight).to.equal(
      partAWeight.toString(),
      "Weight is not correct"
    );
    expect(metadataDiscriminant).to.equal(
      mintDiscriminant.toString(),
      "Discriminant is not correct"
    );
    expect(metadataCollection).to.equal(
      collectionPda.toBase58(),
      "Collection is not correct"
    );
    expect(mint2.decimals).to.equal(0, "Decimals is not correct");
    expect(mint2.supply).to.equal(BigInt(1), "Supply is not correct");
    expect(mint2.isInitialized).to.equal(true, "Mint Authority is not correct");
    expect(mint2Metadata.name).to.equal(
      createMintMetadata.name,
      "Name is not correct"
    );
    expect(mint2Metadata.symbol).to.equal(
      createMintMetadata.symbol,
      "Symbol is not correct"
    );
    expect(mint2Metadata.uri).to.equal(
      createMintMetadata.uri,
      "Uri is not correct"
    );
    expect(metadataWeight2).to.equal(
      partBWeight.toString(),
      "Weight is not correct"
    );
    expect(metadataDiscriminant2).to.equal(
      mint2Discriminant.toString(),
      "Discriminant is not correct"
    );
    expect(metadataCollection2).to.equal(
      collectionPda.toBase58(),
      "Collection is not correct"
    );
  });
  // ... existing code ...

  it("List NFT", async () => {
    const mintDiscriminant = new anchor.BN(0);
    const price = new anchor.BN(1_0000); // 100 Dollars

    const [mintPda, mintPdaBump] = getPda(program, Pda.Mint, [
      mintDiscriminant.toArrayLike(Buffer, "le", 8),
    ]);
    const [listingPda, listingPdaBump] = getPda(program, Pda.Listing, [
      mintPda.toBuffer(),
      program.provider.publicKey.toBuffer(),
    ]);
    const [listingTokenAccountPda, listingTokenAccountPdaBump] = getPda(
      program,
      Pda.ListingTokenAccount,
      [listingPda.toBuffer()]
    );
    const [userAccountPda, userAccountPdaBump] = getPda(
      program,
      Pda.UserAccount,
      [program.provider.publicKey.toBuffer()]
    );

    const listNftIx = await program.methods
      .listNft({ discriminant: mintDiscriminant, price })
      .accountsPartial({
        mint: mintPda,
        // collection: collectionPda,
        listing: listingPda,
        // listingTokenAccount: listingTokenAccountPda,
      })
      .instruction();
    const createUserAccountIx = await program.methods
      .createUserAccount()
      .instruction();

    const tx = new anchor.web3.Transaction()
      .add(listNftIx)
      .add(createUserAccountIx);

    await program.provider.sendAndConfirm(tx);

    const listing = await program.account.listing.fetch(listingPda);
    const user = await program.account.user.fetch(userAccountPda);

    expect(listing.price.eq(price)).to.be.true;
    expect(listing.mint.equals(mintPda)).to.be.true;
    expect(listing.owner.equals(program.provider.publicKey)).to.be.true;
    expect(user.authority.equals(program.provider.publicKey)).to.be.true;
    expect(user.bump).to.equal(userAccountPdaBump);
  });

  it("Buy NFT", async () => {
    const mintDiscriminant = new anchor.BN(0);
    const buyer = newAuthority;

    const [mintPda] = getPda(program, Pda.Mint, [
      mintDiscriminant.toArrayLike(Buffer, "le", 8),
    ]);
    const [listingPda] = getPda(program, Pda.Listing, [
      mintPda.toBuffer(),
      program.provider.publicKey.toBuffer(),
    ]);

    const buyNftIx = await program.methods
      .buyNft(mintDiscriminant)
      .accountsPartial({
        buyer: buyer.publicKey,
        seller: program.provider.publicKey,
        mint: mintPda,
        listing: listingPda,
        solPriceUpdate: solPriceUpdateKey,
        recipient: program.provider.publicKey,
      })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(buyNftIx),
      [buyer]
    );

    // Verify the NFT was transferred
    const recipientTokenAccount =
      await program.provider.connection.getTokenAccountsByOwner(
        program.provider.publicKey,
        { mint: mintPda }
      );
    expect(recipientTokenAccount.value.length).to.equal(1);
  });

  it("Update Listing Price", async () => {
    const mintDiscriminant = new anchor.BN(1); // Use second NFT
    const initialPrice = new anchor.BN(100_000); // 1000 Dollars
    const updatedPrice = new anchor.BN(200_000); // 2000 Dollars

    const [mintPda] = getPda(program, Pda.Mint, [
      mintDiscriminant.toArrayLike(Buffer, "le", 8),
    ]);
    const [listingPda] = getPda(program, Pda.Listing, [
      mintPda.toBuffer(),
      program.provider.publicKey.toBuffer(),
    ]);

    // List the NFT first
    const listNftIx = await program.methods
      .listNft({ discriminant: mintDiscriminant, price: initialPrice })
      .accountsPartial({
        mint: mintPda,
        listing: listingPda,
      })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(listNftIx)
    );

    // Update the listing price
    const updateListingPriceIx = await program.methods
      .updateListingPrice({
        discriminant: mintDiscriminant,
        newPrice: updatedPrice,
      })
      .accountsPartial({
        mint: mintPda,
        listing: listingPda,
      })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(updateListingPriceIx)
    );

    // Verify the updated price
    const listing = await program.account.listing.fetch(listingPda);
    expect(listing.price.eq(updatedPrice)).to.be.true;
  });

  it("Delist NFT", async () => {
    const mintDiscriminant = new anchor.BN(1); // Use second NFT

    const [mintPda] = getPda(program, Pda.Mint, [
      mintDiscriminant.toArrayLike(Buffer, "le", 8),
    ]);
    const [listingPda] = getPda(program, Pda.Listing, [
      mintPda.toBuffer(),
      program.provider.publicKey.toBuffer(),
    ]);

    const delistNftIx = await program.methods
      .delistNft(mintDiscriminant)
      .accountsPartial({
        mint: mintPda,
        listing: listingPda,
      })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(delistNftIx)
    );

    // Verify listing was removed
    try {
      await program.account.listing.fetch(listingPda);
      expect.fail("Listing should be closed");
    } catch (e) {
      expect(e.message).to.include("Account does not exist");
    }
  });

  it("Update Metadata", async () => {
    const mintDiscriminant = new anchor.BN(0); // Use first NFT
    const updatedMetadata = {
      name: "Updated NFT Name",
      symbol: "UPD",
      uri: "https://updated-uri.com",
    };

    const [mintPda] = getPda(program, Pda.Mint, [
      mintDiscriminant.toArrayLike(Buffer, "le", 8),
    ]);

    const updateMetadataNameIx = await program.methods
      .updateMetadata({
        discriminant: mintDiscriminant,

        field: {
          name: {},
        },
        value: updatedMetadata.name,
      })
      .accountsPartial({
        mint: mintPda,
      })
      .instruction();
    const updateMetadataSymbolIx = await program.methods
      .updateMetadata({
        discriminant: mintDiscriminant,
        field: {
          symbol: {},
        },
        value: updatedMetadata.symbol,
      })
      .accountsPartial({
        mint: mintPda,
      })
      .instruction();
    const updateMetadataUriIx = await program.methods
      .updateMetadata({
        discriminant: mintDiscriminant,
        field: {
          uri: {},
        },
        value: updatedMetadata.uri,
      })
      .accountsPartial({
        mint: mintPda,
      })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction()
        .add(updateMetadataNameIx)
        .add(updateMetadataSymbolIx)
        .add(updateMetadataUriIx)
    );

    // Verify updated metadata
    const mintMetadata = await getTokenMetadata(
      program.provider.connection,
      mintPda,
      "processed",
      TOKEN_2022_PROGRAM_ID
    );

    expect(mintMetadata.name).to.equal(
      updatedMetadata.name,
      "Name is not correct"
    );
    expect(mintMetadata.symbol).to.equal(
      updatedMetadata.symbol,
      "Symbol is not correct"
    );
    expect(mintMetadata.uri).to.equal(
      updatedMetadata.uri,
      "URI is not correct"
    );
  });

  it("Burn NFT", async () => {
    const mintDiscriminant = new anchor.BN(1);

    const [mintPda] = getPda(program, Pda.Mint, [
      mintDiscriminant.toArrayLike(Buffer, "le", 8),
    ]);

    const burnNftIx = await program.methods
      .burnNft(mintDiscriminant)
      .accountsPartial({
        mint: mintPda,
      })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(burnNftIx)
    );

    // Verify mint account was closed
    const mintInfo = await program.provider.connection.getAccountInfo(mintPda);
    expect(mintInfo).to.be.null;
  });

  it("User Withdraw", async () => {
    const withdrawAmount = new anchor.BN(500_000_000); // 0.5 SOL
    const balBefore = await program.provider.connection.getBalance(
      program.provider.publicKey
    );

    const userWithdrawIx = await program.methods
      .userWithdraw(withdrawAmount)
      // .accountsPartial({
      //   user: userAccountPda,
      // })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(userWithdrawIx)
    );

    const balAfter = await program.provider.connection.getBalance(
      program.provider.publicKey
    );
    const difference = balAfter - balBefore;
    expect(difference).to.be.greaterThan(0, "Balance did not increase");
    expect(difference).to.be.greaterThanOrEqual(
      0.45 * anchor.web3.LAMPORTS_PER_SOL,
      "Balance increased by more than expected"
    );
  });

  it("Admin Withdraw Fees", async () => {
    const withdrawAmount = new anchor.BN(20_000_000); // 0.1 SOL

    const adminBalBefore = await program.provider.connection.getBalance(
      program.provider.publicKey
    );

    const adminWithdrawFeesIx = await program.methods
      .adminWithdrawFees(withdrawAmount)
      .accounts({
        recipient: program.provider.publicKey,
      })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(adminWithdrawFeesIx)
    );

    const adminBalAfter = await program.provider.connection.getBalance(
      program.provider.publicKey
    );

    const difference = adminBalAfter - adminBalBefore;
    expect(difference).to.be.greaterThan(0, "Balance did not increase");
    expect(difference).to.be.greaterThanOrEqual(
      0.018 * anchor.web3.LAMPORTS_PER_SOL,
      "Balance increased by more than expected"
    );
  });

  it("Admin Withdraw Mint Fees", async () => {
    const withdrawAmount = new anchor.BN(500_000_000); // 0.5 SOL

    const adminBalBefore = await program.provider.connection.getBalance(
      program.provider.publicKey
    );

    const adminWithdrawMintFeesIx = await program.methods
      .adminWithdrawMintFees(withdrawAmount)
      .accounts({ recipient: program.provider.publicKey })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(adminWithdrawMintFeesIx)
    );

    const adminBalAfter = await program.provider.connection.getBalance(
      program.provider.publicKey
    );

    const difference = adminBalAfter - adminBalBefore;
    expect(difference).to.be.greaterThan(0, "Balance did not increase");
    expect(difference).to.be.greaterThanOrEqual(
      0.45 * anchor.web3.LAMPORTS_PER_SOL,
      "Balance increased by more than expected"
    );
  });

  it("Update Fees", async () => {
    const newFractionalizeFee = 500; // 0.05%
    const newSellFee = 1000; // 0.1%

    const updateFeesFractionalizeIx = await program.methods
      .updateFees({
        fee: { fractionalizeFee: {} },
        newFee: newFractionalizeFee,
      })
      .instruction();

    const updateFeesSellIx = await program.methods
      .updateFees({
        fee: { sellFee: {} },
        newFee: newSellFee,
      })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction()
        .add(updateFeesFractionalizeIx)
        .add(updateFeesSellIx)
    );

    const feesCollectorData = await program.account.feesCollector.fetch(
      feesCollectorPda
    );

    expect(feesCollectorData.fractionalizeFee).to.equal(
      newFractionalizeFee,
      "Fractionalize Fee is not updated correctly"
    );
    expect(feesCollectorData.sellFee).to.equal(
      newSellFee,
      "Sell Fee is not updated correctly"
    );
  });

  it("Initialize Transfer Ownership", async () => {
    const initializeTransferOwnershipIx = await program.methods
      .initiailizeOwnershipTransfer()
      .accounts({
        newOwner: newAuthority.publicKey,
      })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(initializeTransferOwnershipIx)
    );

    const nftManagerData = await program.account.nftManager.fetch(
      nftManagerPda
    );

    expect(nftManagerData.futureAuthority.toBase58()).to.equal(
      newAuthority.publicKey.toBase58(),
      "Future Authority is not set correctly"
    );
  });

  it("Finalize Ownership Transfer", async () => {
    // Simulate setting the future authority
    const finalizeOwnershipTransferIx = await program.methods
      .finalizeOwnershipTransfer()
      .accounts({ signer: newAuthority.publicKey })
      .instruction();

    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(finalizeOwnershipTransferIx),
      [newAuthority]
    );

    const nftManagerData = await program.account.nftManager.fetch(
      nftManagerPda
    );

    expect(nftManagerData.authority.toBase58()).to.equal(
      newAuthority.publicKey.toBase58(),
      "Ownership transfer was not finalized correctly"
    );
    expect(nftManagerData.futureAuthority).to.be.null;
  });
});
