import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stake } from "../target/types/stake";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

describe("stake", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.stake as Program<Stake>;

  let stake;
  let user_stake;
  let stakePda: PublicKey;
  let stakeBump: number;
  let vaultPda: PublicKey;
  let vaultBump: number;
  let vaultAuthorityPda: PublicKey;
  let vaultAuthorityBump: number;
  let userStakePda: PublicKey;
  let userStakeBump: number;
  let userAta;

  const metadata = {
    name: "Solana Gold",
    symbol: "SOL",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
  };

  let staking_mint: PublicKey = new PublicKey(
    "57dQxpHFknJs96w1Z1DTHi6QgxmR9i7XdhxKuZp8xtzQ"
  );

  let mintKeypair = new Keypair();
  let admin = provider.wallet.publicKey;
  let reward_rate = new anchor.BN(4);
  let user = new PublicKey("5KpEiTZ7vpuUZdrEcgY524MsU5cJR85gfHW25YAsX81E");

  // it("Create SPL token!", async () => {
  //   const tx = await program.methods
  //     .createTokenMint(9, metadata.name, metadata.symbol, metadata.uri)
  //     .accounts({
  //       payer: payer.publicKey,
  //       mintAccount: mintKeypair.publicKey,
  //     })
  //     .signers([mintKeypair])
  //     .rpc();

  //   console.log("Success!");
  //   console.log(`Mint address: ${mintKeypair.publicKey} `);
  //   console.log(`Transaction signature ${tx}`);
  // });

  // it("Mint SPL token to payers wallet", async () => {
  //   const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
  //     mintKeypair.publicKey,
  //     payer.publicKey
  //   );
  //   const amount = new anchor.BN(100);

  //   const tx = await program.methods
  //     .mintToken(amount)
  //     .accounts({
  //       mintAuthority: payer.publicKey,
  //       recipient: payer.publicKey,
  //       mintAccount: mintKeypair.publicKey,
  //       associatedTokenAccount: associatedTokenAccountAddress,
  //     })
  //     .rpc();
  //   console.log("SUCCESS!");
  //   console.log(
  //     `Associated TOken Account Address: ${associatedTokenAccountAddress}`
  //   );
  //   console.log(`Transaction Signature ${tx}`);
  //   console.log(payer.publicKey);
  // });

  it("Should be able to create a stake", async () => {
    [stakePda, stakeBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("stake"), admin.toBuffer()],
      program.programId
    );

    [vaultPda, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), stakePda.toBuffer()],
      program.programId
    );

    [vaultAuthorityPda, vaultAuthorityBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vault_authority"), vaultPda.toBuffer()],
        program.programId
      );

    const stakeAccount = await provider.connection.getAccountInfo(stakePda);
    console.log("Stake account before:", stakeAccount);

    if (stakeAccount === null) {
      await program.methods
        .createStake(reward_rate)
        .accounts({
          admin: admin,
          stake: stakePda,
          vault: vaultPda,
          vaultAuthority: vaultAuthorityPda,
          stakingMint: staking_mint,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      stake = await program.account.adminStake.fetch(stakePda);
      console.log("Stake after", stakePda);
    } else {
      console.log("Stake account already exists");
    }
  });

  it("User should be able to stake token", async () => {
    [userStakePda, userStakeBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user_stake"), user.toBuffer(), stakePda.toBuffer()],
        program.programId
      );

    const balance = await provider.connection.getBalance(user);
    console.log("The balance of user is :", balance);

    // Creating a ATA of the USer for the staking token.
    userAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      staking_mint,
      user
    );

    // mint to user (ALready done)
    // await mintTo(
    //   provider.connection,
    //   payer.payer,
    //   staking_mint,
    //   userAta.address,
    //   payer.publicKey,
    //   1000_000_000_000
    // );
    const userTokenAccountInfo = await getAccount(
      provider.connection,
      userAta.address
    );
    console.log(`Recipient balance: ${userTokenAccountInfo.amount}`);

    const amount_to_stake = new anchor.BN(1);
    const time_to_stake = new anchor.BN(1);

    await program.methods.stakeToken(amount_to_stake, time_to_stake).accounts({
      user: user,
    });
  });
});
