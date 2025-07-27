## zkHack Challenge #1
*Challenge: https://zkhack.dev/events/puzzle1.html*

### Relevant Background

#### BLS Signature Scheme

**Premise**

Suppose we have groups $G_1$, $G_2$, $G_T$ equipped with:

- *a bilinear pairing function* $e: G_1 \times G_2 \rightarrow G_T$
- *a uniform-random hash function* $H: \{0, 1\}^* \rightarrow G_2$

Now, we define a signature scheme where:
- *the private key is given by* $sk \in \mathbb{Z}_p$
- *the public key is given by* $pk = sk \cdot g_2 \in G_2$

> **🔒 Security Assumption:**  
> The *Discrete Logarithm (DLOG) Assumption* ensures that it is computationally infeasible to recover $sk$ from $pk$.

---

**Signing**

Suppose we want to sign a message $m$.  


We map $m$ onto a point in $G_2$ by taking $H(m)$.  
From this, we sign the message by calculating the signature:  

$$\sigma = sk \cdot H(m) \in G_2$$

---

**Verification**

We want to verify that $\sigma$ was signed with the secret key $sk$ corresponding to $pk$.  
Essentially, we are posing the question: Does $\sigma = sk \cdot H(m)$ hold?

But $sk$ is secret, so we check this equality *indirectly* by using the pairing:

$$e(\sigma, g_2) \stackrel{?}{=} e(H(m), pk)$$

*since we have the following by bilinearity of the pairing function:*

$$e(\sigma, g_2) = e(sk \cdot H(m), g_2) = e(H(m), g_2)^{sk} = e(H(m), sk \cdot g_2) = e(H(m), pk)$$

---

**Aggregation**

This signature scheme also supports signature aggregation:

$$\sigma_{\text{agg}} = \sum_{i=1}^n \sigma_i$$

> **Note:** This aggregated signature $\sigma_{\text{agg}}$ is a group element (constant-size) no matter how many signatures were combined.

To verify the aggregated signature, we check the following:

$$
e(\sigma_{\text{agg}}, g_2) \stackrel{?}{=} \prod_{i=1}^n e(H(m_i), pk_i)
$$

*since we have the following by bilinearity of the pairing function:*

$$
\begin{aligned}
e(\sigma_{\text{agg}}, g_2) &= e\left(\sum_{i=1}^n \sigma_i, g_2\right) \\\\
                            &= \prod_{i=1}^n e(\sigma_i, g_2) \\\\
                            &= \prod_{i=1}^n e(sk_i \cdot H(m_i), g_2) \\\\
                            &= \prod_{i=1}^n e(H(m_i), sk_i \cdot g_2) \\\\
                            &= \prod_{i=1}^n e(H(m_i), pk_i)
\end{aligned}
$$

---

#### Pedersen Hash

The Pedersen Hash is given by the following:

$$\text{PedersenHash}(x_1, \dots, x_n) = \sum_{i=1}^{n} x_i \cdot G_i$$
*Where:*
- $x_i \in \{0,1\}$ are bits of the message $m$
- $G_i$ are fixed public elliptic curve points

**Linearity**

The Pedersen hash is *linear* in its inputs:

$$
\text{PedersenHash}(x + y) = \text{PedersenHash}(x) + \text{PedersenHash}(y)
$$

*Because:*

$$
\begin{aligned}
\text{PedersenHash}(x_1 + y_1, \dots, x_n + y_n) &= \sum_{i=1}^n (x_i + y_i) \cdot G_i \\
                                                  &= \sum_{i=1}^n x_i \cdot G_i + \sum_{i=1}^n y_i \cdot G_i \\
                                                  &= \text{PedersenHash}(x) + \text{PedersenHash}(y)
\end{aligned}
$$


---

### The Exploit

> Since the hash function and the signature scheme are both *linear*, an attacker, who knows a set of messages and a set of corresponding signatures (all signed by the same secret key), can forge a signature on a linear combination of those messages without knowing the secret key.

Suppose the attacker has messages $m_1, m_2, \dots, m_n$ (where each message is represented in bits as $m_i = (x_{i1}, x_{i2}, \dots, x_{il})$) and their corresponding signatures $\sigma_1, \sigma_2, \dots, \sigma_n$.

The attacker chooses coefficients $\alpha_1, \alpha_2, \dots, \alpha_n \in \mathbb{Z}_p$ and forms a new message:

$$
m^* = \sum_{i=1}^n \alpha_i m_i = \left( \sum_{i=1}^n \alpha_i x_{i1}, \sum_{i=1}^n \alpha_i x_{i2}, \dots, \sum_{i=1}^n \alpha_i x_{il} \right)
$$

Now, the attacker can forge the signature for $m^*$ as follows:

*Using the linearity of the hash function:*

$$
\begin{aligned}
H(m^*) &= \sum_{j=1}^l \left(\sum_{i=1}^n \alpha_i x_{ij}\right) G_j \\
       &= \sum_{i=1}^n \alpha_i \left(\sum_{j=1}^l x_{ij} G_j \right) \\
       &= \sum_{i=1}^n \alpha_i H(m_i)
\end{aligned}
$$

*Using the linearity of the signature scheme:*

$$
\begin{aligned}
\sigma^* &= sk \cdot H(m^*) \\
         &= sk \cdot \sum_{i=1}^n \alpha_i H(m_i) \\
         &= \sum_{i=1}^n \alpha_i (sk \cdot H(m_i)) \\
         &= \sum_{i=1}^n \alpha_i \sigma_i
\end{aligned}
$$

Observe that the forged signature $\sigma^*$ passes verification:

$$
\begin{aligned}
e(\sigma^*, g_2) &= e\left(\sum_{i=1}^n \alpha_i \sigma_i, g_2\right) \\
                 &= \prod_{i=1}^n e(\sigma_i, g_2)^{\alpha_i} \\
                 &= \prod_{i=1}^n e(H(m_i), pk)^{\alpha_i} \\
                 &= e\left(\sum_{i=1}^n \alpha_i H(m_i), pk\right) \\
                 &= e(H(m^*), pk)
\end{aligned}
$$

---

#### Commands

```rust
cargo run --bin preprocessing
```

```rust
sage solve_coeffs.sage
```

```rust
cargo run --bin verify-bls-pedersen
```