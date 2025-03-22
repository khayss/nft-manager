import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftManager } from "../target/types/nft_manager";

export enum Pda {
  Mint,
  Collection,
  FeesCollector,
  MintFeesCollector,
  NftManager,
  Listing,
  ListingTokenAccount,
  UserAccount,
}

export enum Metadata {
  Discriminant,
  Collection,
  Weight,
}

export function getPda(
  program: Program<NftManager>,
  pda: Pda,
  extra_seeds?: Uint8Array[]
): [anchor.web3.PublicKey, number] {
  const seeds = [];
  switch (pda) {
    case Pda.NftManager:
      seeds.push(Uint8Array.from(JSON.parse(program.idl.constants[12].value)));
      break;
    case Pda.FeesCollector:
      seeds.push(Uint8Array.from(JSON.parse(program.idl.constants[4].value)));
      break;
    case Pda.MintFeesCollector:
      seeds.push(Uint8Array.from(JSON.parse(program.idl.constants[10].value)));
      break;
    case Pda.Collection:
      seeds.push(Uint8Array.from(JSON.parse(program.idl.constants[1].value)));
      break;
    case Pda.Mint:
      seeds.push(Uint8Array.from(JSON.parse(program.idl.constants[11].value)));
      break;
    case Pda.Listing:
      seeds.push(Uint8Array.from(JSON.parse(program.idl.constants[7].value)));

      break;
    case Pda.ListingTokenAccount:
      seeds.push(Uint8Array.from(JSON.parse(program.idl.constants[14].value)));
      break;
    case Pda.UserAccount:
      seeds.push(Uint8Array.from(JSON.parse(program.idl.constants[13].value)));
      break;
    default:
      throw new Error("Invalid Pda");
  }
  if (extra_seeds) {
    for (const seed of extra_seeds) {
      seeds.push(seed);
    }
  }

  return anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
}

export function getAdditionMetadata(
  metadata: Metadata,
  additionalMetaData: (readonly [string, string])[]
): string {
  for (const [key, value] of additionalMetaData) {
    switch (metadata) {
      case Metadata.Collection:
        if (key === "collection") {
          return value;
        }
        break;
      case Metadata.Weight:
        if (key === "weight") {
          return value;
        }
        break;

      case Metadata.Discriminant:
        if (key === "discriminant") {
          return value;
        }
        break;
    }
  }
  throw new Error("Invalid Metadata");
}
export const collectionName = "Collection";
export const collectionSymbol = "COL";
export const collectionUri = "https://collection.com";

const solPriceUpdateStr = "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE";
const goldPriceUpdateStr = "2uPQGpm8X4ZkxMHxrAW1QuhXcse1AHEgPih6Xp9NuEWW";

export const solPriceUpdateKey = new anchor.web3.PublicKey(solPriceUpdateStr);
export const goldPriceUpdateKey = new anchor.web3.PublicKey(goldPriceUpdateStr);

export const fractionalizeFee = 150;
export const sellFee = 250;

export const createMintMetadata: {
  name: string;
  symbol: string;
  uri: string;
} = {
  name: "JP Gold NFT",
  symbol: "JPGC",
  uri: "https://jpgc.com",
};
