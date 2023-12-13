import path from "path";
import os from "os";
import fs from "fs";
import * as anchor from "@coral-xyz/anchor";

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
