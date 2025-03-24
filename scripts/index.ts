import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftManager } from "../target/types/nft_manager";
import * as fs from "fs";

const homeDir = process.env.HOME;

const rawKeypair = JSON.parse(
  fs.readFileSync(homeDir + "/.config/solana/id.json", "utf-8")
);
const rawKeypair2 = JSON.parse(
  fs.readFileSync(homeDir + "/.config/solana/phantom.json", "utf-8")
);

const idlJson = JSON.parse(
  fs.readFileSync("./target/idl/nft_manager.json", "utf-8")
);

const keyPairUint8 = Uint8Array.from(rawKeypair);
const keypair = anchor.web3.Keypair.fromSecretKey(keyPairUint8);
const keyPair2 = anchor.web3.Keypair.fromSecretKey(
  Uint8Array.from(rawKeypair2)
);
export const wallet = new anchor.Wallet(keypair);
export const wallet2 = new anchor.Wallet(keyPair2);

// Setup connection and provider
const connection = new anchor.web3.Connection("https://api.devnet.solana.com", {
  commitment: "confirmed",
  wsEndpoint:
    "wss://devnet.helius-rpc.com/?api-key=c7ac5a23-e869-4406-a6d7-983783bcc657",
});
// "wss://api.devnet.solana.com",
// "https://devnet.helius-rpc.com/?api-key=c7ac5a23-e869-4406-a6d7-983783bcc657",

const provider = new anchor.AnchorProvider(connection, wallet, {
  commitment: "confirmed",
});

// Initialize program
export const nftManagerProgram = new anchor.Program(
  idlJson,
  provider
) as Program<NftManager>;
const SOL_PRICE_FEED_ADDR_STR = "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE";
const GOLD_PRICE_FEED_ADDR_STR = "2uPQGpm8X4ZkxMHxrAW1QuhXcse1AHEgPih6Xp9NuEWW";

export const solPriceFeed = new anchor.web3.PublicKey(SOL_PRICE_FEED_ADDR_STR);
export const goldPriceFeed = new anchor.web3.PublicKey(
  GOLD_PRICE_FEED_ADDR_STR
);
