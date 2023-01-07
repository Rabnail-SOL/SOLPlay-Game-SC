import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DepositGame } from "../target/types/deposit_game";

async function sendLamports(
  provider,
  destination,
  amount
) {
  const tx = new anchor.web3.Transaction();
  tx.add(
      anchor.web3.SystemProgram.transfer(
          { 
              fromPubkey: provider.wallet.publicKey, 
              lamports: amount, 
              toPubkey: destination
          }
      )
  );
  
  var signature = await anchor.web3.sendAndConfirmTransaction(
    provider.connection,
    tx,
    [provider.wallet.payer]
  );
}

describe("deposit-game", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  let poolKeypair = anchor.web3.Keypair.generate();
  let poolPubkey = poolKeypair.publicKey;

  let user1 = anchor.web3.Keypair.generate();
  let user2 = anchor.web3.Keypair.generate();
  let user3 = anchor.web3.Keypair.generate();
  let user4 = anchor.web3.Keypair.generate();
  let user5 = anchor.web3.Keypair.generate();
  let user6 = anchor.web3.Keypair.generate();
  let user7 = anchor.web3.Keypair.generate();
  let user8 = anchor.web3.Keypair.generate();
  let user9 = anchor.web3.Keypair.generate();
  let user10 = anchor.web3.Keypair.generate();
  let admin = anchor.web3.Keypair.generate();
  let gameId1;

  const program = anchor.workspace.DepositGame as Program<DepositGame>;

  it("Is initialized!", async () => {
    
    await sendLamports(provider, user1.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user2.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user3.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user4.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user5.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user6.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user7.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user8.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user9.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user10.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, admin.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);


    const [
        poolSigner,
        nonce,
    ] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          poolPubkey.toBuffer(),
        ],
        program.programId
    );
    
    const tx = await program.methods.initialize(nonce).accounts({
      authority: provider.wallet.publicKey,
      pool: poolPubkey,
      poolSigner: poolSigner,
      owner: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([poolKeypair, ]).rpc();
    console.log("Your transaction signature", tx);
  });

  
  it('create game 1', async () => {
    const envProvider = anchor.AnchorProvider.env();
    const p = new anchor.AnchorProvider(envProvider.connection, new anchor.Wallet(admin), envProvider.opts);

    const userProgram = new anchor.Program(program.idl, program.programId, p);

    let poolObject = await userProgram.account.poolAccount.fetch(poolPubkey);
    gameId1 = (poolObject.gameCount.toNumber() + 1).toString();

    const [
        game,
        nonce,
    ] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          poolPubkey.toBuffer(), Buffer.from("game"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const [
        vault,
        nonceVault,
    ] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          poolPubkey.toBuffer(), Buffer.from("vault"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    let gameObject;
    try {
        gameObject = await userProgram.account.gameAccount.fetch(game);
    } catch (e) {
        console.log(e.message)
    }
    let instructions = [];
    if(!gameObject) {
        instructions.push(
            await userProgram.methods.createGame(nonce, nonceVault, gameId1, new anchor.BN(2), new anchor.BN(2), new anchor.BN(anchor.web3.LAMPORTS_PER_SOL))
                                      .accounts({
                                        pool: poolPubkey,
                                        game,
                                        vault,
                                        signer: admin.publicKey,
                                        systemProgram: anchor.web3.SystemProgram.programId,
                                    }).instruction()
        )
    }

    const transaction = new anchor.web3.Transaction().add(...instructions);
      var signature = await anchor.web3.sendAndConfirmTransaction(
        p.connection,
        transaction,
        [admin]
      );
  })

  it('deposit1', async () => {
    const user = user1;
    const envProvider = anchor.AnchorProvider.env();
    const p = new anchor.AnchorProvider(envProvider.connection, new anchor.Wallet(user), envProvider.opts);

    const userProgram = new anchor.Program(program.idl, program.programId, p);

    const [
        _poolSigner,
        _nonce,
    ] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          poolPubkey.toBuffer(),
        ],
        userProgram.programId
    );

    const [
        game,
    ] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          poolPubkey.toBuffer(), Buffer.from("game"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const [
        vault,
    ] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          poolPubkey.toBuffer(), Buffer.from("vault"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const [
      deposit,
    ] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          user.publicKey.toBuffer(), Buffer.from("deposit"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    let contractLamports1 = (await provider.connection.getBalance(vault));
    console.log("vault value: ", contractLamports1);
    let adminLamports1 = (await provider.connection.getBalance(admin.publicKey));
    console.log("admin value: ", adminLamports1);
    let userLamports1 = (await provider.connection.getBalance(user.publicKey));
    console.log("user value: ", userLamports1);
    const gameObject = await userProgram.account.gameAccount.fetch(game);
    const tx = await userProgram.methods.deposit().accounts({
        pool: poolPubkey,
        game,
        vault,
        depositor: user.publicKey,
        feeReceiver: gameObject.feeReceiver,
        poolSigner: _poolSigner,
        deposit,
        signer: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();

    let contractLamports = (await provider.connection.getBalance(vault));
    console.log("vault value: ", contractLamports);
    let adminLamports = (await provider.connection.getBalance(admin.publicKey));
    console.log("admin value: ", adminLamports);
    let userLamports = (await provider.connection.getBalance(user.publicKey));
    console.log("user value: ", userLamports);
    // assert.equal(contractLamports, amount);
  })

});
