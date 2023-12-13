import * as anchor from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";
import { AssetsManager } from "../target/types/assets_manager";
import { BN, Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { getDefaultWallet } from "./helpers";

describe("assets-manager", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AssetsManager as Program<AssetsManager>;
  const [globalState] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("global_state")],
    program.programId
  );

  const defaultWallet = getDefaultWallet();
  let tokenMint: anchor.web3.PublicKey;
  let assetInfo: anchor.web3.PublicKey;
  let asset: anchor.web3.PublicKey;

  before(async () => {
    tokenMint = await token.createMint(
      provider.connection,
      defaultWallet,
      provider.wallet.publicKey,
      provider.wallet.publicKey,
      6
    );

    [assetInfo] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("asset_info"), tokenMint.toBuffer()],
      program.programId
    );
    [asset] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("asset"), tokenMint.toBuffer()],
      program.programId
    );
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        globalState,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const globalStateAccount = await program.account.globalState.fetch(
      globalState
    );
    assert(
      globalStateAccount.admin.toBase58() ===
        anchor.getProvider().publicKey.toBase58()
    );
  });

  it("Add new asset", async () => {
    const tx = await program.methods
      .addAsset(new BN(1000000))
      .accounts({
        globalState,
        underlyingToken: tokenMint,
        assetInfo,
        asset,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const assetInfoAccount = await program.account.assetInfo.fetch(assetInfo);
    assert(
      assetInfoAccount.underlyingToken.toBase58() === tokenMint.toBase58()
    );
    assert(assetInfoAccount.asset.toBase58() === asset.toBase58());
  });
});
