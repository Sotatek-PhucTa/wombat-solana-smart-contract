import path from "path";
import os from "os";
import fs from "fs";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { web3 } from "@coral-xyz/anchor";

export function getDefaultWallet(filePath?: string) {
  const dataPath = filePath
    ? filePath
    : path.join(os.homedir(), ".config/solana/id.json");
  console.log("data path ", dataPath);
  const data = fs.readFileSync(dataPath);
  return anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(data.toString()))
  );
}
export function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function airdrop(connection: web3.Connection, address: PublicKey) {
  await connection.requestAirdrop(address, 10000000000);
  await delay(5000);
}
