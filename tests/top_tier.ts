import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { TopTier } from "../target/types/top_tier";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";

const userA = Keypair.generate();
const userB = Keypair.generate();

describe("top_tier", () => {
  const title = "Some Entry";

  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.topTier as Program<TopTier>;

  it("Is initialized!", async () => {
    await provider.connection.requestAirdrop(userA.publicKey, 1e9);
    await provider.connection.requestAirdrop(userB.publicKey, 1e9);
    await program.methods.initialize().rpc();
  });

  it("can add entries!", async () => {
    const metadata = "https://example.com/metadata";

    const metadataBytes = new TextEncoder().encode(metadata);
    const titleBytes = new TextEncoder().encode(title);

    const fixedMetadataBytes = new Uint8Array(128); // To fixed-size [u8; 128]
    const fixedTitleBytes = new Uint8Array(32); // To fixed-size [u8; 32]
    fixedMetadataBytes.set(metadataBytes);
    fixedTitleBytes.set(titleBytes);

    const [leaderboardPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("leaderboard")],
      program.programId
    );

    const tx = await program.methods
      .addEntry([...fixedTitleBytes], [...fixedMetadataBytes])
      .accounts({
        leaderboard: leaderboardPda,
        signer: userA.publicKey,
      } as any) // TODO: ask why errors on type
      .signers([userA])
      .rpc();
  });

  it("can read entries!", async () => {
    const [leaderboardPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("leaderboard")],
      program.programId
    );

    const leaderboard = await program.account.leaderBoard.fetch(leaderboardPda);

    // Decode entries
    leaderboard.entries.forEach((entry, index) => {
      if (entry.score > 0) {
        // only show non-empty entries
        const uri = new TextDecoder()
          .decode(new Uint8Array(entry.metadataUri))
          .replace(/\0/g, "");
        console.log(`Entry ${index}: uri=${uri}, score=${entry.score}`);
      }
    });
  });

  it("can vote for entries!", async () => {
    const titleBytes = new TextEncoder().encode(title);

    const fixedTitleBytes = new Uint8Array(32);

    fixedTitleBytes.set(titleBytes);

    const [leaderboardPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("leaderboard")],
      program.programId
    );

    const [voteRecordPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote"),
        userB.publicKey.toBuffer(),
        Buffer.from(fixedTitleBytes),
      ],
      program.programId
    );

    await program.methods
      .vote([...fixedTitleBytes])
      .accounts({
        leaderboard: leaderboardPda,
        voteRecord: voteRecordPda,
        voter: userB.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([userB])
      .rpc();

    const leaderboard = await program.account.leaderBoard.fetch(leaderboardPda);

    leaderboard.entries.forEach((entry, index) => {
      if (entry.score > 0) {
        const uri = new TextDecoder()
          .decode(new Uint8Array(entry.metadataUri))
          .replace(/\0/g, "");
        console.log(`Entry ${index}: uri=${uri}, score=${entry.score}`);
      }
    });
  });

  it("errors in case leaderboard is full", async () => {
    const [leaderboardPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("leaderboard")],
      program.programId
    );

    for (let i = 1; i < 32; i++) {
      const metadata = `https://example.com/metadata${i}`;
      const titleForEntry = `Entry ${i}`;

      const metadataBytes = new TextEncoder().encode(metadata);
      const titleBytes = new TextEncoder().encode(titleForEntry);

      const fixedMetadataBytes = new Uint8Array(128);
      const fixedTitleBytes = new Uint8Array(32);
      fixedMetadataBytes.set(metadataBytes);
      fixedTitleBytes.set(titleBytes);

      const tx = await program.methods
        .addEntry([...fixedTitleBytes], [...fixedMetadataBytes])
        .accounts({
          leaderboard: leaderboardPda,
          signer: userA.publicKey,
        } as any)
        .signers([userA])
        .rpc();
    }

    const metadata = `https://example.com/metadata${33}`;
    const titleForEntry = `Entry ${33}`;

    const metadataBytes = new TextEncoder().encode(metadata);
    const titleBytes = new TextEncoder().encode(titleForEntry);

    const fixedMetadataBytes = new Uint8Array(128);
    const fixedTitleBytes = new Uint8Array(32);
    fixedMetadataBytes.set(metadataBytes);
    fixedTitleBytes.set(titleBytes);

    try {
      await program.methods
        .addEntry([...fixedTitleBytes], [...fixedMetadataBytes])
        .accounts({
          leaderboard: leaderboardPda,
          signer: userA.publicKey,
        } as any)
        .signers([userA])
        .rpc();

      expect.fail("Should have thrown error");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("LeaderboardFull");
    }
  });

  it("can sort entries", async () => {
    const titleForEntry = "Entry 31";
    const titleBytes = new TextEncoder().encode(titleForEntry);

    const fixedTitleBytes = new Uint8Array(32);

    fixedTitleBytes.set(titleBytes);

    const [leaderboardPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("leaderboard")],
      program.programId
    );

    const [voteRecordPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote"),
        userB.publicKey.toBuffer(),
        Buffer.from(fixedTitleBytes),
      ],
      program.programId
    );

    await program.methods
      .vote([...fixedTitleBytes])
      .accounts({
        leaderboard: leaderboardPda,
        voteRecord: voteRecordPda,
        voter: userB.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([userB])
      .rpc();

    const leaderboard = await program.account.leaderBoard.fetch(leaderboardPda);

    leaderboard.entries.forEach((entry, index) => {
      if (entry.score > 0) {
        const uri = new TextDecoder()
          .decode(new Uint8Array(entry.metadataUri))
          .replace(/\0/g, "");
        console.log(`Entry ${index}: uri=${uri}, score=${entry.score}`);
      }
    });
  });
});
