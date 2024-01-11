import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Pyth } from "../target/types/pyth";
import { PublicKey} from "@solana/web3.js";
import { BN } from "bn.js";

describe("pyth", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Pyth as Program<Pyth>;
  const wallet = anchor.workspace.Pyth.provider.wallet.payer;
  const usdc_amount = new BN(50)
  it("Send money", async () => {
    // Add your test here.
    const tx = await program.methods.payUsd(usdc_amount)
      .accounts({solUsdPriceAccount: new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"), to : new PublicKey("7wK3jPMYjpZHZAghjersW6hBNMgi9VAGr75AhYRqR2n")})
      .signers([wallet])
      .rpc();
    console.log(`Txn complete: https://solana.fm/tx/${tx}?cluster=devnet-solana`)
  });
});