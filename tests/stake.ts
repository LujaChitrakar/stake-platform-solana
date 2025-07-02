import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stake } from "../target/types/stake";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

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

  // let staking_mint;

  let mintKeypair = new Keypair();
  let admin = provider.wallet.publicKey;
  let reward_rate = new anchor.BN(100);
  // let user = Keypair.generate();

  let secret_key = "YOUR SECRET KEY";

  let secret_key_bytes = bs58.decode(secret_key);

  let user = anchor.web3.Keypair.fromSecretKey(secret_key_bytes);

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
        [
          Buffer.from("user_stake"),
          user.publicKey.toBuffer(),
          stakePda.toBuffer(),
        ],
        program.programId
      );

    // await provider.connection.confirmTransaction(
    //   await provider.connection.requestAirdrop(user.publicKey, 1e9),
    //   "confirmed"
    // );

    const balance = await provider.connection.getBalance(user.publicKey);
    console.log("The balance of user is :", balance);

    // Creating a ATA of the USer for the staking token.
    userAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      staking_mint,
      user.publicKey
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

    console.log("User ATA:", userStakePda.toBase58());

    await program.methods
      .stakeToken(amount_to_stake, time_to_stake)
      .accounts({
        user: user.publicKey,
        stake: stakePda,
        user_stake: userStakePda,
        vault: vaultPda,
        user_ata: userAta.address,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    await new Promise((resolve) => setTimeout(resolve, 10000));

    user_stake = await program.account.userStake.fetch(userStakePda);

    console.log("user staked", user_stake);
  });

  it("Should be able to unstake token", async () => {
    await program.methods
      .unstakeToken()
      .accounts({
        user: user.publicKey,
        stake: stakePda,
        user_stake: userStakePda,
        vault: vaultPda,
        vault_authority: vaultAuthorityPda,
        user_ata: userAta.address,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    await new Promise((resolve) => setTimeout(resolve, 10000));
    user_stake = await program.account.userStake.fetch(userStakePda);
    console.log("User Unstaked", user_stake);
  });

  it("Should generate reward after some time", async () => {
    await new Promise((resolve) => setTimeout(resolve, 10000));
    user_stake = await program.account.userStake.fetch(userStakePda);
    console.log("User Unstaked", user_stake);
  });

  it("Should be able to claim reward", async () => {
    await program.methods
      .claimReward()
      .accounts({
        user: user.publicKey,
        stake: stakePda,
        user_stake: userStakePda,
        vault: vaultPda,
        vault_authority: vaultAuthorityPda,
        user_ata: userAta.address,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    user_stake = await program.account.userStake.fetch(userStakePda);
    console.log("Reward claimed", user_stake);
  });
});
