// @ts-nocheck
// This is a client to interact with the simple counter program.
// Feel free to test it on Solana Playground.
// NOTE: If you're switching from a simple counter to global counter,
// don't forget to remove `payer.publicKey.toBuffer()` from the PDA seed.


import {
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";

// Solana Playground global variables
const programId = new PublicKey("FNuYKdEuhwGFp23UuQk9P4Hh9VktotebvrR6Xnmd2m4S");
const payer = pg.wallet.keypair;
const connection = pg.connection;

// Derive the PDA
const [pda, _] = await PublicKey.findProgramAddressSync(
  [Buffer.from("simple_counter"), payer.publicKey.toBuffer()],
  programId
);

// Define the accounts required by the program instruction
const keys = [
  { pubkey: pda, isSigner: false, isWritable: true },
  { pubkey: payer.publicKey, isSigner: true, isWritable: false },
  { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
];

// Instruction data (empty buffer since no data needed for this instruction)
const data = Buffer.alloc(0);

// Fetch the latest blockhash to construct the transaction
const blockhashInfo = await pg.connection.getLatestBlockhash();

// Create the transaction
const tx = new web3.Transaction({ ...blockhashInfo });
tx.add(
  new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
);

// Sign and send the transaction
tx.sign(payer);
const txHash = await connection.sendRawTransaction(tx.serialize());
console.log("✅ Sent to program with PDA:", pda.toBase58());
console.log("🔁 Transaction hash:", txHash);
