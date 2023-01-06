import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DepositGame } from "../target/types/deposit_game";

describe("deposit-game", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DepositGame as Program<DepositGame>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
