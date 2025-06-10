// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PromptVault } from "../target/types/prompt_vault";
import { PublicKey, SystemProgram } from "@solana/web3.js";

module.exports = async function (provider: anchor.AnchorProvider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  const program = anchor.workspace.PromptVault as Program<PromptVault>;
  
  console.log("Deploying PromptHub Protocol...");
  console.log("Program ID:", program.programId.toString());
  
  // Derive vault state PDA
  const [vaultState] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault_state")],
    program.programId
  );
  
  console.log("Vault State PDA:", vaultState.toString());
  
  try {
    // Check if vault is already initialized
    const vaultStateAccount = await program.account.vaultState.fetchNullable(vaultState);
    
    if (vaultStateAccount) {
      console.log("Vault already initialized!");
      console.log("Admin:", vaultStateAccount.admin.toString());
      console.log("Fee BPS:", vaultStateAccount.feeBps);
      console.log("Min Stake Amount:", vaultStateAccount.minStakeAmount.toString());
      return;
    }
    
    // Initialize vault with default parameters
    const admin = provider.wallet.publicKey;
    const feeBps = 1000; // 10%
    const minStakeAmount = new anchor.BN(1000 * 10**9); // 1000 tokens
    
    console.log("Initializing vault...");
    console.log("Admin:", admin.toString());
    console.log("Fee BPS:", feeBps);
    console.log("Min Stake Amount:", minStakeAmount.toString());
    
    const tx = await program.methods
      .initialize(admin, feeBps, minStakeAmount)
      .accounts({
        vaultState,
        admin,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    
    console.log("Vault initialized successfully!");
    console.log("Transaction signature:", tx);
    
    // Verify initialization
    const initializedVault = await program.account.vaultState.fetch(vaultState);
    console.log("Verification - Admin:", initializedVault.admin.toString());
    console.log("Verification - Fee BPS:", initializedVault.feeBps);
    console.log("Verification - Is Paused:", initializedVault.isPaused);
    
  } catch (error) {
    console.error("Deployment failed:", error);
    throw error;
  }
  
  console.log("PromptHub Protocol deployment completed!");
}; 