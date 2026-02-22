import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { TopTier } from "../target/types/top_tier";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";

const userA = Keypair.generate();
const userB = Keypair.generate();

async function logLeaderBoard(leaderboard) {
  for (let i = 0; i < leaderboard.count.toNumber(); i++) {
    const entry = leaderboard.entries[i];

    const entryData = await program.account.entry.fetch(entry.pubkey);
    console.log(`#${i + 1}: },title: ${entryData.title}, score=${entry.score}`);
  }
}

async function checkEntry(title: String) {
  const [entryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("entry"), Buffer.from(title)],
    program.programId
  );
  const entry = await program.account.entry.fetch(entryPda);
  // console.log("Title:", entry.title);
  // console.log("URI:", entry.metadataUri);
  // console.log("Score:", entry.score.toNumber());
  // console.log("Creator:", entry.creator.toString());
  return entry;
}

async function voteForEntry(title: String, leaderboardPda, user) {
  const [entryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("entry"), Buffer.from(title)],
    program.programId
  );

  const [voteRecordPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("vote"), user.publicKey.toBuffer(), entryPda.toBuffer()],
    program.programId
  );

  const tx = await program.methods
    .vote()
    .accounts({
      leaderboard: leaderboardPda,
      entry: entryPda,
      voteRecord: voteRecordPda,
      voter: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    } as any)
    .signers([user])
    .rpc();

  return tx;
}

anchor.setProvider(anchor.AnchorProvider.env());
const provider = anchor.AnchorProvider.env();
const program = anchor.workspace.topTier as Program<TopTier>;

describe("top_tier", () => {
  const title = "Entry 0";

  it("Is initialized!", async () => {
    await provider.connection.requestAirdrop(userA.publicKey, 1e9);
    await provider.connection.requestAirdrop(userB.publicKey, 1e9);
    await program.methods.initialize().rpc();
  });

  it("can create entries!", async () => {
    const metadata = "https://example.com/metadata";

    // const metadataBytes = new TextEncoder().encode(metadata);
    // const titleBytes = new TextEncoder().encode(title);

    // const fixedMetadataBytes = new Uint8Array(128); // To fixed-size [u8; 128]
    // const fixedTitleBytes = new Uint8Array(32); // To fixed-size [u8; 32]
    // fixedMetadataBytes.set(metadataBytes);
    // fixedTitleBytes.set(titleBytes);

    const [leaderboardPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("leaderboard")],
      program.programId
    );

    await program.methods
      .createEntry(title, metadata)
      .accounts({
        leaderboard: leaderboardPda,
        creator: userA.publicKey,
      } as any) // TODO: ask why errors on type
      .signers([userA])
      .rpc();

    const entry = await checkEntry(title);

    expect(entry.title).to.equal(title);
    expect(entry.score.toNumber()).to.equal(0);
  });

  it("can vote for entries!", async () => {
    const [leaderboardPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("leaderboard")],
      program.programId
    );

    await voteForEntry(title, leaderboardPda, userA);

    const entry = await checkEntry(title);

    expect(entry.score.toNumber()).to.equal(1);
  });

  it("Entry can enter full leaderboard", async () => {
    const [leaderboardPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("leaderboard")],
      program.programId
    );

    for (let i = 1; i < 32; i++) {
      const metadata = `https://example.com/metadata${i}`;
      const titleForEntry = `Entry ${i}`;

      await program.methods
        .createEntry(titleForEntry, metadata)
        .accounts({
          leaderboard: leaderboardPda,
          creator: userA.publicKey,
        } as any)
        .signers([userA])
        .rpc();

      await voteForEntry(titleForEntry, leaderboardPda, userA);
    }

    let leaderboard = await program.account.leaderBoard.fetch(leaderboardPda);
    const firstEntryBefore = leaderboard.entries[0];

    const firstEntryBeforeData = await program.account.entry.fetch(
      firstEntryBefore.pubkey
    );
    expect(firstEntryBeforeData.title).to.equal("Entry 0"); // check if first entry in the list is the first one added

    const metadata33 = `https://example.com/metadata${33}`;
    const titleForEntry33 = `Entry ${33}`;

    await program.methods
      .createEntry(titleForEntry33, metadata33)
      .accounts({
        leaderboard: leaderboardPda,
        creator: userA.publicKey,
      } as any)
      .signers([userA])
      .rpc();

    await voteForEntry(titleForEntry33, leaderboardPda, userA);
    await voteForEntry(titleForEntry33, leaderboardPda, userB);

    leaderboard = await program.account.leaderBoard.fetch(leaderboardPda);
    const firstEntryAfter = leaderboard.entries[0];

    const firstEntryAfterData = await program.account.entry.fetch(
      firstEntryAfter.pubkey
    );
    expect(firstEntryAfterData.title).to.equal("Entry 33"); // check if first entry in the list is the last one added
  });

  // it.skip("can sort entries", async () => {
  //   const titleForEntry = "Entry 31";
  //   const titleBytes = new TextEncoder().encode(titleForEntry);

  //   const fixedTitleBytes = new Uint8Array(32);

  //   fixedTitleBytes.set(titleBytes);

  //   const [leaderboardPda] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("leaderboard")],
  //     program.programId
  //   );

  //   const [voteRecordPda] = PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.from("vote"),
  //       userB.publicKey.toBuffer(),
  //       Buffer.from(fixedTitleBytes),
  //     ],
  //     program.programId
  //   );

  //   await program.methods
  //     .vote([...fixedTitleBytes])
  //     .accounts({
  //       leaderboard: leaderboardPda,
  //       voteRecord: voteRecordPda,
  //       voter: userB.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     } as any)
  //     .signers([userB])
  //     .rpc();

  //   const leaderboard = await program.account.leaderBoard.fetch(leaderboardPda);

  //   leaderboard.entries.forEach((entry, index) => {
  //     if (entry.score > 0) {
  //       const uri = new TextDecoder()
  //         .decode(new Uint8Array(entry.metadataUri))
  //         .replace(/\0/g, "");
  //       console.log(`Entry ${index}: uri=${uri}, score=${entry.score}`);
  //     }
  //   });
  // });
});
