import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PumpfunIntegrateRaydium } from "../target/types/pumpfun_integrate_raydium";

describe("pumpfun-integrate-raydium", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.pumpfunIntegrateRaydium as Program<PumpfunIntegrateRaydium>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
