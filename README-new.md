# ZK Hack Puzzle 1 — Solution Explanation

## Overview

This puzzle challenges you to forge a valid BLS signature on a *target message* using only a set of **256 known message-signature pairs** signed under the *same secret key* and the public key.

The key insight is to exploit the **linearity properties** of the BLS signature scheme combined with the **Pedersen hash** used to map messages to elliptic curve points.

---

## Background Concepts

### BLS Signature Scheme

- We have groups \(G_1\), \(G_2\), and a bilinear pairing function:

  \[
  e : G_1 \times G_2 \to G_T
  \]

- The private key \(sk \in \mathbb{Z}_p\).
- The public key \(pk = sk \cdot g_2 \in G_2\).
- Messages \(m\) are hashed to points in \(G_2\) using a hash function \(H\).
- Signing:

  \[
  \sigma = sk \cdot H(m) \in G_2
  \]

- Verification checks the bilinear pairing equation:

  \[
  e(\sigma, g_2) \stackrel{?}{=} e(H(m), pk)
  \]

---

### Pedersen Hash

The hash function \(H\) here is a **Pedersen hash**:

\[
\text{PedersenHash}(x_1, \ldots, x_n) = \sum_{i=1}^n x_i \cdot G_i
\]

where \(x_i\) are bits of the message (or derived from its digest), and \(G_i\) are fixed elliptic curve points.

This hash is **linear over the bit-vector input**, i.e.,

\[
H(m_1) + H(m_2) = H(m_1 \oplus m_2)
\]

(when \(\oplus\) is the bitwise addition modulo the curve order).

---

## Aggregation and Linearity

- Signatures can be aggregated by summing:

  \[
  \sigma_{\text{agg}} = \sum_i \sigma_i = \sum_i sk_i \cdot H(m_i)
  \]

- Verification works on aggregated signatures because of bilinearity:

  \[
  e(\sigma_{\text{agg}}, g_2) = \prod_i e(H(m_i), pk_i)
  \]

- **Critical assumption**: public keys \(pk_i\) are independent in \(G_2\).

---

## Rogue Key Attack Exploit

- If an attacker crafts a malicious key as a **linear combination** of honest public keys:

  \[
  pk_{\text{attacker}} = pk_2 - pk_1
  \]

- Then the attacker can forge a signature on some message \(m_{\text{attacker}}\) **without knowing** \(sk_{\text{attacker}}\) by setting:

  \[
  \sigma_{\text{attacker}} = \sigma_2 - \sigma_1
  \]

- Aggregated signature and key become:

  \[
  \sigma_{\text{agg}} = \sigma_1 + \sigma_{\text{attacker}} = \sigma_2
  \]
  \[
  pk_{\text{agg}} = pk_1 + pk_{\text{attacker}} = pk_2
  \]

- Verification passes without knowing the secret key of the attacker.

---

## How This Applies to Puzzle 1

- You have 256 messages \(\{m_j\}\) and signatures \(\{\sigma_j\}\), all signed by the *same secret key*.
- Each message is hashed using Pedersen hash on the 256-bit Blake2s digest of the message.
- Because of the **linearity of the Pedersen hash and BLS signature**, a **linear combination** of the known signatures corresponds to the signature of the **linear combination** of their bit-vectors.
- Your goal:  
  Find coefficients \(c_j\) such that

  \[
  H(m_{\text{target}}) = \sum_{j=1}^{256} c_j \cdot H(m_j)
  \]

- This becomes a linear algebra problem over the finite field \(\mathbb{F}_r\) of size equal to the BLS scalar field:

  \[
  A \cdot \mathbf{c} = \mathbf{b}
  \]

  where:
  - \(A\) is a \(256 \times 256\) matrix whose columns are the bit-vectors of known messages' Blake2s digests,
  - \(\mathbf{b}\) is the bit-vector of the target message’s digest,
  - \(\mathbf{c}\) are the unknown coefficients to solve.

- Since \(A\) is invertible, you compute:

  \[
  \mathbf{c} = A^{-1} \cdot \mathbf{b}
  \]

- Then forge the signature:

  \[
  \sigma_{\text{target}} = \sum_{j=1}^{256} c_j \cdot \sigma_j
  \]

- The forged signature \(\sigma_{\text{target}}\) will pass verification under the public key without knowing the secret key.

---

## Practical Steps

1. **Hash all known messages** with Blake2s, convert to 256-bit vectors.
2. **Hash the target message** the same way.
3. Construct matrix \(A\) and vector \(\mathbf{b}\).
4. Solve the linear system in \(\mathbb{F}_r\).
5. Use the coefficients to compute the forged signature from the known signatures.
6. Verify using the existing public key and target message.

---

## Commands to Run

```bash
cargo run --bin extract_hash_vectors  # output bit matrix and target vector CSVs
sage solve_coeffs.sage > selectors.rs # solve coefficients in Sage and output Rust code
