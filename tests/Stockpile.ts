import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Stockpile } from "../target/types/stockpile";

describe("Stockpile", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Stockpile as Program<Stockpile>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
