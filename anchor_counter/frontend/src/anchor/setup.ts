import type { IdlAccounts } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { Counter } from "./counter.ts";
import { Counter as CounterIDL } from "./counter.ts";
import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";

// Use the correct program ID from your IDL
const programId = new PublicKey("Adf9VCdjngGn8f6RZftLNFqc6zVUKs3JF5rvqXzZLwaQ");
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

// Initialize the program interface with the IDL, program ID, and connection.
// This setup allows us to interact with the on-chain program using the defined interface.
export const program = new Program<Counter>(CounterIDL as Counter, {
  connection,
});

// Derive the counter PDA using the same seed as defined in the IDL
export const [counterPDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("counter")],
  programId,
);

// This is just a TypeScript type for the Counter data structure based on the IDL
// We need this so TypeScript doesn't yell at us
export type CounterData = IdlAccounts<Counter>["counter"];