import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { programs } from "@metaplex/js";
import { MetaChange } from "../target/types/meta_change";

describe("meta-change", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MetaChange as Program<MetaChange>;

  it("Update authority", async () => {
    const nftPubkey = new PublicKey("NFT Public Key to update authority here");
    const newAuth = new PublicKey("New Update Authority Public key here");
    const tokenMetadataProgram = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    const pdaAddress = await programs.metadata.Metadata.getPDA(nftPubkey);

    const tx = await program.methods.updateAuthority().accounts({
      metadataAccount: pdaAddress,
      updateAuthority: (program.provider as anchor.AnchorProvider).wallet
      .publicKey,
      newUpdateAuthority: newAuth,
      tokenMetadataProgram: tokenMetadataProgram
    }).rpc();
    
    console.log("Your transaction signature", tx);
  });
});
