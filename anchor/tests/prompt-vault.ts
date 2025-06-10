import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PromptVault } from "../target/types/prompt_vault";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo } from "@solana/spl-token";
import { expect } from "chai";

describe("prompt-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PromptVault as Program<PromptVault>;
  const provider = anchor.getProvider();

  let admin: Keypair;
  let user: Keypair;
  let mint: PublicKey;
  let adminTokenAccount: PublicKey;
  let userTokenAccount: PublicKey;
  let vaultState: PublicKey;

  before(async () => {
    // Initialize keypairs
    admin = Keypair.generate();
    user = Keypair.generate();

    // Airdrop SOL
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(admin.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL)
    );

    // Create mint
    mint = await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      9
    );

    // Create token accounts
    adminTokenAccount = await createAccount(
      provider.connection,
      admin,
      mint,
      admin.publicKey
    );

    userTokenAccount = await createAccount(
      provider.connection,
      user,
      mint,
      user.publicKey
    );

    // Mint tokens
    await mintTo(
      provider.connection,
      admin,
      mint,
      adminTokenAccount,
      admin,
      1000000 * 10**9
    );

    await mintTo(
      provider.connection,
      admin,
      mint,
      userTokenAccount,
      admin,
      1000000 * 10**9
    );

    // Derive vault state PDA
    [vaultState] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_state")],
      program.programId
    );
  });

  it("Initialize vault", async () => {
    const feeBps = 1000; // 10%
    const minStakeAmount = 1000 * 10**9; // 1000 tokens

    await program.methods
      .initialize(admin.publicKey, feeBps, new anchor.BN(minStakeAmount))
      .accounts({
        vaultState,
        admin: admin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    const vaultStateAccount = await program.account.vaultState.fetch(vaultState);
    expect(vaultStateAccount.admin.toString()).to.equal(admin.publicKey.toString());
    expect(vaultStateAccount.feeBps).to.equal(feeBps);
    expect(vaultStateAccount.minStakeAmount.toString()).to.equal(minStakeAmount.toString());
  });

  it("Register a prompt", async () => {
    const promptId = "test-prompt-1";
    const title = "Test Prompt";
    const description = "A test prompt for demonstration";
    const contentUri = "ipfs://QmTestHash123";
    const tags = ["test", "demo"];
    const licenseType = { public: {} };
    const price = 100 * 10**9; // 100 tokens

    const [promptData] = PublicKey.findProgramAddressSync(
      [Buffer.from("prompt"), Buffer.from(promptId)],
      program.programId
    );

    await program.methods
      .registerPrompt(
        promptId,
        title,
        description,
        contentUri,
        tags,
        licenseType,
        new anchor.BN(price)
      )
      .accounts({
        promptData,
        vaultState,
        author: user.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const promptDataAccount = await program.account.promptData.fetch(promptData);
    expect(promptDataAccount.id).to.equal(promptId);
    expect(promptDataAccount.title).to.equal(title);
    expect(promptDataAccount.author.toString()).to.equal(user.publicKey.toString());
  });

  it("Record execution", async () => {
    const promptId = "test-prompt-1";
    const executionId = "exec-123";
    const success = true;
    const executionTimeMs = 1500;
    const signature = Buffer.from("test-signature");

    const [promptData] = PublicKey.findProgramAddressSync(
      [Buffer.from("prompt"), Buffer.from(promptId)],
      program.programId
    );

    const [executionRecord] = PublicKey.findProgramAddressSync(
      [Buffer.from("execution"), Buffer.from(executionId)],
      program.programId
    );

    await program.methods
      .recordExecution(executionId, success, executionTimeMs, Array.from(signature))
      .accounts({
        promptData,
        executionRecord,
        vaultState,
        caller: user.publicKey,
        callerTokenAccount: userTokenAccount,
        authorTokenAccount: adminTokenAccount, // Using admin account as author
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const executionRecordAccount = await program.account.executionRecord.fetch(executionRecord);
    expect(executionRecordAccount.executionId).to.equal(executionId);
    expect(executionRecordAccount.success).to.equal(success);
    expect(executionRecordAccount.executionTimeMs).to.equal(executionTimeMs);
  });

  it("Create version", async () => {
    const promptId = "test-prompt-1";
    const version = "1.1.0";
    const contentUri = "ipfs://QmNewVersionHash456";
    const changelog = "Added new features and bug fixes";

    const [promptData] = PublicKey.findProgramAddressSync(
      [Buffer.from("prompt"), Buffer.from(promptId)],
      program.programId
    );

    await program.methods
      .createVersion(version, contentUri, changelog)
      .accounts({
        promptData,
        author: user.publicKey,
      })
      .signers([user])
      .rpc();

    const promptDataAccount = await program.account.promptData.fetch(promptData);
    expect(promptDataAccount.currentVersion).to.equal(version);
    expect(promptDataAccount.contentUri).to.equal(contentUri);
  });

  it("Stake for ranking", async () => {
    const promptId = "test-prompt-1";
    const stakeAmount = 5000 * 10**9; // 5000 tokens

    const [promptData] = PublicKey.findProgramAddressSync(
      [Buffer.from("prompt"), Buffer.from(promptId)],
      program.programId
    );

    const [stakeAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("stake"), Buffer.from(promptId), user.publicKey.toBuffer()],
      program.programId
    );

    const [stakePool] = PublicKey.findProgramAddressSync(
      [Buffer.from("stake_pool")],
      program.programId
    );

    await program.methods
      .stakeForRanking(new anchor.BN(stakeAmount))
      .accounts({
        stakeAccount,
        promptData,
        vaultState,
        staker: user.publicKey,
        stakerTokenAccount: userTokenAccount,
        stakePool,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const stakeAccountData = await program.account.stakeAccount.fetch(stakeAccount);
    expect(stakeAccountData.stakedAmount.toString()).to.equal(stakeAmount.toString());
    expect(stakeAccountData.owner.toString()).to.equal(user.publicKey.toString());
  });

  it("Emergency pause and resume", async () => {
    // Pause
    await program.methods
      .emergencyPause()
      .accounts({
        vaultState,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    let vaultStateAccount = await program.account.vaultState.fetch(vaultState);
    expect(vaultStateAccount.isPaused).to.be.true;

    // Resume
    await program.methods
      .resumeOperations()
      .accounts({
        vaultState,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    vaultStateAccount = await program.account.vaultState.fetch(vaultState);
    expect(vaultStateAccount.isPaused).to.be.false;
  });

  it("Update metadata", async () => {
    const promptId = "test-prompt-1";
    const newTitle = "Updated Test Prompt";
    const newDescription = "Updated description for the test prompt";
    const newTags = ["updated", "test", "demo"];

    const [promptData] = PublicKey.findProgramAddressSync(
      [Buffer.from("prompt"), Buffer.from(promptId)],
      program.programId
    );

    await program.methods
      .updateMetadata(newTitle, newDescription, newTags)
      .accounts({
        promptData,
        author: user.publicKey,
      })
      .signers([user])
      .rpc();

    const promptDataAccount = await program.account.promptData.fetch(promptData);
    expect(promptDataAccount.title).to.equal(newTitle);
    expect(promptDataAccount.description).to.equal(newDescription);
    expect(promptDataAccount.tags).to.deep.equal(newTags);
  });

  it("Fork prompt", async () => {
    const originalPromptId = "test-prompt-1";
    const newPromptId = "forked-prompt-1";
    const newTitle = "Forked Test Prompt";
    const newDescription = "A forked version of the original prompt";

    const [originalPromptData] = PublicKey.findProgramAddressSync(
      [Buffer.from("prompt"), Buffer.from(originalPromptId)],
      program.programId
    );

    const [newPromptData] = PublicKey.findProgramAddressSync(
      [Buffer.from("prompt"), Buffer.from(newPromptId)],
      program.programId
    );

    await program.methods
      .forkPrompt(newPromptId, newTitle, newDescription)
      .accounts({
        originalPromptData,
        newPromptData,
        vaultState,
        forker: admin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    const newPromptDataAccount = await program.account.promptData.fetch(newPromptData);
    expect(newPromptDataAccount.id).to.equal(newPromptId);
    expect(newPromptDataAccount.title).to.equal(newTitle);
    expect(newPromptDataAccount.author.toString()).to.equal(admin.publicKey.toString());
  });
}); 