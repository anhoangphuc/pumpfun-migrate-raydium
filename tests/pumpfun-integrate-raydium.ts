import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PumpfunIntegrateRaydium } from "../target/types/pumpfun_integrate_raydium";
import { ComputeBudgetInstruction, ComputeBudgetProgram, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {getAssociatedTokenAddressSync, NATIVE_MINT, syncNative, TOKEN_PROGRAM_ID} from '@solana/spl-token'
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
  });

  it("Swap success", async () => {
    const tx = await program.methods.swap().accounts({
      mint: mint.publicKey,
      signer: signer.publicKey,
    }).signers([signer])
    .rpc({ commitment: 'confirmed' });
    console.log("Swap success", tx);

    const [vaultAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from('vault'), mint.publicKey.toBuffer()],
       program.programId
    );

    const vaultTokenAddress = getAssociatedTokenAddressSync(NATIVE_MINT, vaultAddress, true);
    const vaultTokenBalance = await connection.getTokenAccountBalance(vaultTokenAddress, 'confirmed');
    assert.equal(vaultTokenBalance.value.amount, '20000000000', 'Vault token balance should be 20000000000');
  })

});
