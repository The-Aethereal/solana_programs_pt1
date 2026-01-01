import type { IdlAccounts } from "@coral-xyz/anchor";

import { Program } from "@coral-xyz/anchor";
import type { Counter } from "./idl";
import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";
import CounterState from "../components/counter-state";
import { IDL } from "@coral-xyz/anchor/dist/cjs/native/system";

const programId = new PublicKey("Adf9VCdjngGn8f6RZftLNFqc6zVUKs3JF5rvqXzZLwaQ");
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

// Initialize the program interface with the IDL, program ID, and connection.
// This setup allows us to interact with the on-chain program using the defined interface.
export const program = new Program<Counter>(IDL, programId, {
  connection,
});

export const [counterPDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("counter")],
  program.programId,
);

// This is just a TypeScript type for the Counter data structure based on the IDL
// We need this so TypeScript doesn't yell at us
export type CounterData = IdlAccounts<Counter>["counter"];