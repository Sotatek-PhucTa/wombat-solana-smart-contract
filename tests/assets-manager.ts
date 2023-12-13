import * as anchor from "@coral-xyz/anchor";
import { AssetsManager } from "../target/types/assets_manager";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";

describe("assets-manager", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.AssetsManager as Program<AssetsManager>;
  const [globalState, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("global_state")],
    program.programId
  );

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
});
