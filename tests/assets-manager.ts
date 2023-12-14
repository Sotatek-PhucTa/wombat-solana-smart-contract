import * as anchor from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";
import { AssetsManager } from "../target/types/assets_manager";
import { BN, Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { airdrop, delay, getDefaultWallet } from "./helpers";
import * as punycode from "punycode";

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

    const globalStateAccount = await program.account.globalState.fetch(
      globalState
    );
    assert(
      globalStateAccount.admin.toBase58() ===
        anchor.getProvider().publicKey.toBase58()
    );
  });

  it("Add new asset failed unauthorized", async () => {
    try {
      const signer = anchor.web3.Keypair.generate();
      await airdrop(provider.connection, signer.publicKey);
      const tx = await program.methods
        .addAsset(new BN(1000000))
        .accounts({
          globalState,
          underlyingToken: tokenMint,
          assetInfo,
          asset,
          signer: signer.publicKey,
        })
        .signers([signer])
        .rpc();
    } catch (error) {
      assert(error.error.errorCode.code === "ConstraintAddress");
    }
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

    const assetInfoAccount = await program.account.assetInfo.fetch(assetInfo);
    assert(
      assetInfoAccount.underlyingToken.toBase58() === tokenMint.toBase58()
    );
    assert(assetInfoAccount.asset.toBase58() === asset.toBase58());
  });

  it("Set max supply unauthorized", async () => {
    try {
      const signer = anchor.web3.Keypair.generate();
      await airdrop(provider.connection, signer.publicKey);
      await program.methods
        .setMaxSupply(new BN(2000000))
        .accounts({
          underlyingToken: tokenMint,
          assetInfo,
          globalState,
          signer: signer.publicKey,
        })
        .signers([signer])
        .rpc();
    } catch (error) {
      assert(error.error.errorCode.code === "ConstraintAddress");
    }
  });

  it("Set max supply", async () => {
    await program.methods
      .setMaxSupply(new BN(2000000))
      .accounts({
        underlyingToken: tokenMint,
        assetInfo,
        globalState,
      })
      .rpc();

    const assetInfoAccount = await program.account.assetInfo.fetch(assetInfo);
    assert(assetInfoAccount.maxSupply.eq(new BN(2000000)));
  });

  it("Transfer underlying token", async () => {
    let assetInfoAta = await token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      defaultWallet,
      tokenMint,
      assetInfo,
      true
    );
    let userAta = await token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      defaultWallet,
      tokenMint,
      defaultWallet.publicKey
    );
    const tx = await token.mintTo(
      provider.connection,
      defaultWallet,
      tokenMint,
      assetInfoAta.address,
      defaultWallet,
      100000000
    );
    assetInfoAta = await token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      defaultWallet,
      tokenMint,
      assetInfo,
      true
    );

    await program.methods
      .transferUnderlyingToken(new BN(100000))
      .accounts({
        globalState,
        underlyingToken: tokenMint,
        assetInfo,
        from: assetInfoAta.address,
        to: userAta.address,
      })
      .rpc();
    userAta = await token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      defaultWallet,
      tokenMint,
      defaultWallet.publicKey
    );
    assert(userAta.amount.toString() === "100000");
  });

  it("Mint asset", async () => {
    let userAta = await token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      defaultWallet,
      asset,
      defaultWallet.publicKey
    );

    await program.methods
      .mint(new BN(100000))
      .accounts({
        globalState,
        asset,
        to: userAta.address,
        underlyingToken: tokenMint,
        assetInfo,
      })
      .rpc();

    userAta = await token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      defaultWallet,
      asset,
      defaultWallet.publicKey
    );
    assert(userAta.amount.toString() === "100000");
  });

  it("Mint asset failure out of supply", async () => {
    let userAta = await token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      defaultWallet,
      asset,
      defaultWallet.publicKey
    );

    try {
      await program.methods
        .mint(new BN(2000000))
        .accounts({
          globalState,
          asset,
          to: userAta.address,
          underlyingToken: tokenMint,
          assetInfo,
        })
        .rpc();
    } catch (error) {
      assert(error.error.errorCode.code === "MaxSupplyReached");
    }
  });

  it("Add cash successfully", async () => {
    await program.methods
      .addCash(new BN(100000))
      .accounts({
        globalState,
        underlyingToken: tokenMint,
        assetInfo,
      })
      .rpc();
    const assetInfoAccount = await program.account.assetInfo.fetch(assetInfo);
    assert(assetInfoAccount.cash.eq(new BN(100000)));
  });

  it("Remove cash successfully", async () => {
    await program.methods
      .removeCash(new BN(50000))
      .accounts({
        globalState,
        underlyingToken: tokenMint,
        assetInfo,
      })
      .rpc();
    const assetInfoAccount = await program.account.assetInfo.fetch(assetInfo);
    assert(assetInfoAccount.cash.eq(new BN(50000)));
  });
});
