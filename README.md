```markdown
# 🧾 `Intent` (THE main account)

This is the heart. Everything revolves around this.

```rust
pub struct Intent {
    pub id: u64,
    pub user: Pubkey,

    // What the user gives
    pub input_mint: Pubkey,
    pub input_amount: u64,

    // What the user wants
    pub output_mint: Pubkey,
    pub min_output_amount: u64,

    // Who receives output
    pub recipient: Pubkey,

    // Timing
    pub created_at: i64,
    pub expiry: i64,

    // State
    pub status: IntentStatus,

    // Who filled it
    pub solver: Option<Pubkey>,

    pub bump: u8,
}
```

**IntentStatus enum**

```rust
pub enum IntentStatus {
    Open,
    Filled,
    Settled,
    Cancelled,
    Expired,
}
```

### 📌 Why this matters

This struct alone answers:

- **What?**
- **Who?**
- **How much?**
- **By when?**
- **Current state?**

No guessing. No vibes.

---

# 🧰 `Vault` (escrow for user funds)

User funds MUST be locked somewhere.

```rust
pub struct Vault {
    pub intent: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
```

This is usually:

- A PDA
- Token account owned by the program

### 💡 Rule:

> **User funds never move until settlement**

---

# 🤖 `SolverProfile` (optional but realistic)

If solvers are permissioned or staked.

```rust
pub struct SolverProfile {
    pub solver: Pubkey,
    pub stake: u64,
    pub active: bool,
}
```

You can skip this early, but real protocols don't.

---

# 2️⃣ Instructions (THE ONLY ONES YOU NEED)

If you add more than these early, you're overengineering 😤

## 1. `create_intent`

User creates an intent.

```rust
create_intent(
    input_amount,
    min_output_amount,
    expiry
)
```

**What it does:**

- Creates `Intent`
- Creates `Vault`
- Transfers user tokens → vault
- Sets status = `Open`

**❌ Does NOT:**

- Talk about solvers
- Bridge anything
- Execute swaps

This is pure declaration.

---

## 2. `cancel_intent`

User backs out before it's filled.

```rust
cancel_intent()
```

**Checks:**

- Caller is user
- Status == `Open`
- Not expired

**Actions:**

- Return funds
- Status → `Cancelled`

---

## 3. `fill_intent`

Solver claims the intent.

```rust
fill_intent()
```

**Checks:**

- Status == `Open`
- Not expired
- Solver is valid (if permissioned)

**Actions:**

- Marks solver
- Status → `Filled`

### 📌 IMPORTANT

This does NOT release user funds yet.  
Solver hasn't proven anything.

---

## 4. `settle_intent`

Solver proves delivery.

```rust
settle_intent(proof)
```

**Checks:**

- Caller == solver
- Proof is valid
- Output sent to recipient
- Amount >= `min_output_amount`

**Actions:**

- Release vault funds → solver
- Status → `Settled`

**This is where money actually moves.**

---

## 5. `expire_intent`

Anyone can call this.

```rust
expire_intent()
```

**Checks:**

- Current time > expiry
- Status == `Open`

**Actions:**

- Return funds
- Status → `Expired`

⏱ **Time is a first-class citizen here.**

---

# 3️⃣ State Machine (burn this into your brain)

```
CREATE
  ↓
OPEN ─── cancel ───▶ CANCELLED
  │
  │ fill
  ↓
FILLED ─── settle ─▶ SETTLED
  │
  └── timeout ─────▶ EXPIRED
```
```