"use client";
 
import React, { useMemo } from "react";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";


// Default styles that can be overridden by your app
import "@solana/wallet-adapter-react-ui/styles.css";


export default function AppWalletProvider({
  children,
}: {
  children: React.ReactNode;
}) {

  const endpoint = "https://api.devnet.solana.com";
  const wallets = useMemo(
    () => [
    ],
    [endpoint],
  );
 
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>{children}</WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}