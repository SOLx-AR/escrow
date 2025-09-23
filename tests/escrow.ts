import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { BN } from "bn.js";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Escrow as Program<Escrow>;

  it("Make!", async () => {
    // Add your test here.
    const seed = new BN(1234);
    const receive = new BN(1);
    const deposit = new BN(1);
    const maker = anchor.getProvider();
    const mintA = new anchor.web3.PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
    const mintB = new anchor.web3.PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
    const tokenProgram = TOKEN_PROGRAM_ID
    const makerAtaA = getAssociatedTokenAddressSync(mintA, maker.publicKey, true, tokenProgram);
    const [escrow, _bump] = anchor.web3.PublicKey
      .findProgramAddressSync([Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toArrayLike(Buffer, "le", 8)], program.programId);
    const vault = getAssociatedTokenAddressSync(mintA, escrow, true, tokenProgram);

    const accounts = {
      maker: maker.publicKey,
      mintA,
      mintB,
      makerAtaA,
      escrow,
      vault,
      tokenProgram,
      systemProgram: anchor.web3.SystemProgram.programId,
    };

    const tx = await program.methods
      .make(seed,deposit,receive)
      .accountsPartial(accounts)
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
