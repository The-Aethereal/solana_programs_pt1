# Solana Task CRUD Program

A simple CRUD (Create, Read, Update, Delete) smart contract built using **Anchor** .

---

## Features

- Create tasks using PDA-based accounts  
- Read tasks off-chain via RPC  
- Update task description  
- Toggle task completion status  
- Delete tasks and reclaim rent  
- Ownership enforced via signer checks  

---

## Tech Stack

- Solana
- Anchor
- Rust

---

## Task Account Structure

- `owner: Pubkey` — task creator  
- `task_id: u64` — unique task identifier  
- `description: String` — max 280 characters  
- `is_completed: bool` — completion status  
- `last_modified: i64` — unix timestamp  

---

## Instructions

### Create Task
- Inputs: `task_id`, `description`
- Initializes a new PDA-backed task account

### Read Task
- No on-chain instruction
- Fetch account data directly from RPC

### Update Task
- Updates task description

### Toggle Task
- Toggles completion status

### Delete Task
- Closes task account and returns rent to owner

