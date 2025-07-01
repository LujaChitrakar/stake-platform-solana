import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stake } from "../target/types/stake";
import { Keypair, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

describe("stake", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.stake as Program<Stake>;

  const metadata = {
    name: "Solana Gold",
    symbol: "SOL",
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
  };

  const mintKeypair = new Keypair();

  const admin = provider.wallet.publicKey;
  const user = Keypair.generate();
  const reward_mint = PublicKey;
  const reward_rate = new anchor.BN(4);

  it("Create SPL token!", async () => {
    const tx = await program.methods
      .createTokenMint(9, metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Success!");
    console.log(`Mint address: ${mintKeypair.publicKey} `);
    console.log(`Transaction signature ${tx}`);
  });

  it("Mint token to your wallet", async () => {
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
      mintKeypair.publicKey,
      payer.publicKey
    );

    const amount = new anchor.BN(100);

    const tx = await program.methods
      .mintToken(amount)
      .accounts({
        mintAuthority: payer.publicKey,
        recipient: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
        associatedTokenAccount: associatedTokenAccountAddress,
      })
      .rpc();

    console.log("SUCCESS!");
    console.log(
      `Associated TOken Account Address: ${associatedTokenAccountAddress}`
    );
    console.log(`Transaction Signature ${tx}`);
    console.log(payer.publicKey);
  });

  it("Should be able to create a stake", async () => {
    const tx = await program.methods.createStake(reward_mint, reward_rate);
  });
});
