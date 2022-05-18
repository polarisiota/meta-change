import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MetaChange } from "../target/types/meta_change";

describe("meta-change", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MetaChange as Program<MetaChange>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
