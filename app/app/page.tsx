"use client";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import BN from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { useProgram } from "./useProgram";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";

export default function Home() {
  const { connection } = useConnection();
const wallet = useAnchorWallet();
const { program } = useProgram({ connection, wallet });
  return (
    <div className="font-sans grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
      hola solana
      <WalletMultiButton />
      <button
        onClick={() => {(async () => {
          if (!program || !wallet) return
          const maker = wallet
          const seed = new BN(1234);
          const receive = new BN(1);
          const deposit = new BN(1);

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
        }
)()
        }}
      >
        create escrow
      </button>
    </div>
  );
}
