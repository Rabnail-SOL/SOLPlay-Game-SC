import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DepositGame } from "../target/types/deposit_game";

describe("deposit-game", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  let poolKeypair = anchor.web3.Keypair.generate();

  const program = anchor.workspace.DepositGame as Program<DepositGame>;

  it("Is initialized!", async () => {
    const [
        poolSigner,
        nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(),
        ],
        program.programId
    );
    
    const tx = await program.methods.initialize(nonce).accounts({
      authority: provider.wallet.publicKey,
      pool: poolKeypair.publicKey,
      poolSigner: poolSigner,
      owner: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([poolKeypair]).instructions([
          await program.account.pool.createInstruction(poolKeypair, ),
      ]).rpc();
    console.log("Your transaction signature", tx);
  });
});
