import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PumpfunIntegrateRaydium } from "../target/types/pumpfun_integrate_raydium";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {getAssociatedTokenAddressSync} from '@solana/spl-token'
import { assert } from "chai";

describe("pumpfun-integrate-raydium", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.pumpfunIntegrateRaydium as Program<PumpfunIntegrateRaydium>;
  const connection = anchor.getProvider().connection;
  const signer = Keypair.generate();
  const mint = Keypair.generate();

  before(async () => {
    const txHash = await connection.requestAirdrop(signer.publicKey, LAMPORTS_PER_SOL * 100);
    await connection.confirmTransaction(txHash, 'confirmed');
  });


  it("Creates a token!", async () => {
    const tx = await program.methods.createToken().accounts({
      mint: mint.publicKey,
      signer: signer.publicKey,
    }).signers([signer, mint])
    .rpc({ commitment: 'confirmed' });
    console.log("Create a new token success", tx);

    const [vaultAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from('vault'), mint.publicKey.toBuffer()],
       program.programId
    );

    const vaultTokenAddress = getAssociatedTokenAddressSync(mint.publicKey, vaultAddress, true);

    const vaultTokenBalance = await connection.getTokenAccountBalance(vaultTokenAddress, 'confirmed');
    assert.equal(vaultTokenBalance.value.amount, '1000000000', 'Vault token balance should be 1000000000');

    const vaultBalance = await connection.getBalance(vaultAddress, 'confirmed');
    assert.isTrue(vaultBalance >= 20 * LAMPORTS_PER_SOL, 'Vault balance should be at least 20 SOL');
  });
});
