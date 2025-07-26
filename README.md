## zkHack Challenge #1
*Challenge: https://zkhack.dev/events/puzzle1.html*

### Relevant Background

**BLS Signature Scheme**

Suppose we have groups $G_1$, $G_2$, $G_T$ such that we have a bilinear pairing function  
$$
e: G_1 \times G_2 \rightarrow G_T
$$  
and a hash function  
$$
H: \{0, 1\}^* \rightarrow G_2.
$$

The private key is given by: $sk \in \mathbb{Z}_p$.  
The public key is given by: $pk = sk \cdot g_2 \in G_2$.

The discrete logarithm problem ensures that we are unable to recover $sk$ from $pk$.

In this problem, the hash function is the **Pederson Hash**:

$$
\text{PedersenHash}(x_1, \dots, x_n) = \sum_{i=1}^{n} x_i \cdot G_i
$$

where $x_i$ are bits of the message and $G_i$ are fixed elliptic curve points.

Note that the pederson hash is linear meaning:

$$TODO$$

---

**Signing**

Suppose we want to sign a message $m$.  
We map $m$ onto a point in $G_2$ by taking $H(m)$.

We sign the message by calculating the signature  
$$
\sigma = sk \cdot H(m) \in G_2.
$$

---

**Verification**

We are given a message $m$, a signature $\sigma$, and a public key $pk$.  
We want to verify that $\sigma$ was signed with the secret key $sk$ corresponding to $pk$.

Essentially, we are posing the question:  
**Does** $\sigma = sk \cdot H(m)$ **hold**?

But $sk$ is secret, so we check this **indirectly** by "multiplying" both sides via a pairing, which lets us involve the public key (since $pk = sk \cdot g_2$).

This gives:

$$
e(\sigma, g_2) \stackrel{?}{=} e(H(m), pk)
$$

If this equation holds, then $\sigma$ must be of the form $sk \cdot H(m)$, using the **bilinearity** of the pairing:

$$
\begin{aligned}
e(\sigma, g_2) &= e(sk \cdot H(m), g_2) \\
               &= e(H(m), sk \cdot g_2) \\
               &= e(H(m), pk)
\end{aligned}
$$

**Aggregation**

Suppose we have $n$ signers, each with:

- Private key: $sk_i$
- Public key: $pk_i = sk_i \cdot g_2 \in G_2$
- Message: $m_i$
- Signature: $\sigma_i = sk_i \cdot H(m_i) \in G_2$

We can aggregate the signatures as:

$$
\sigma_{\text{agg}} = \sum_{i=1}^n \sigma_i
$$

Verification is done by checking:

$$
e(\sigma_{\text{agg}}, g_2) \stackrel{?}{=} \prod_{i=1}^n e(H(m_i), pk_i)
$$

This works due to the bilinearity of the pairing.

---

**Rogue Key Attack Exploit**

The security of BLS signature aggregation relies on the assumption that each public key is **independent** in the elliptic curve group.

However, if an attacker crafts their public key as a **linear combination** of honest public keys, this assumption breaks down, enabling a **rogue key attack**.

---

**How it works**

Suppose honest signers have public keys $pk_1$ and $pk_2$. An attacker creates a malicious public key as:

$$
pk_{\text{attacker}} = pk_2 - pk_1
$$

Because $pk_{\text{attacker}}$ is a linear combination of $pk_1$ and $pk_2$, it is **linearly dependent** on the honest keys.

---

**Forged aggregation check**

The honest signatures are:

$$
\sigma_1 = sk_1 \cdot H(m_1), \quad \sigma_2 = sk_2 \cdot H(m_2)
$$

The attacker chooses $\sigma_{\text{attacker}}$ as:

$$
\sigma_{\text{attacker}} = sk_{\text{attacker}} \cdot H(m_{\text{attacker}})
$$

But without knowing $sk_{\text{attacker}}$, the attacker forges $\sigma_{\text{attacker}}$ to satisfy:

$$
\sigma_{\text{attacker}} = \sigma_2 - \sigma_1
$$

---

Now, the aggregated signature is:

$$
\sigma_{\text{agg}} = \sigma_1 + \sigma_{\text{attacker}} = \sigma_1 + (\sigma_2 - \sigma_1) = \sigma_2
$$

The aggregated public key is:

$$
pk_{\text{agg}} = pk_1 + pk_{\text{attacker}} = pk_1 + (pk_2 - pk_1) = pk_2
$$

---

**Verification check**

Verification verifies:

$$
e(\sigma_{\text{agg}}, g_2) \stackrel{?}{=} \prod_i e(H(m_i), pk_i)
$$

Here,

$$
e(\sigma_{\text{agg}}, g_2) = e(\sigma_2, g_2)
$$

and

$$
\prod_i e(H(m_i), pk_i) = e(H(m_1), pk_1) \cdot e(H(m_{\text{attacker}}), pk_{\text{attacker}})
$$

Since

$$
pk_{\text{attacker}} = pk_2 - pk_1
$$

we have

$$
e(H(m_{\text{attacker}}), pk_{\text{attacker}}) = e(H(m_{\text{attacker}}), pk_2) \cdot e(H(m_{\text{attacker}}), -pk_1)
$$

By carefully choosing $m_{\text{attacker}}$ and using the linearity of the pairing, the attacker can make the product equal to $e(H(m_2), pk_2)$, making the aggregate verification pass **without knowing $sk_{\text{attacker}}$**.

---

**Commands**

```rust
cargo run --bin preprocessing
```

```rust
sage solve_coeffs.sage
```

```rust
cargo run --bin verify-bls-pedersen
```