---
title: Complete Proofs of Gödel’s Incompleteness Theorems
subtitle: By Byunghan Kim
created_at: 2023-06-28 19:29:45
updated_at: 2023-06-28 19:29:45
---
# Step 0: Preliminary Remarks

We define recursive and recursively enumerable functions and relations, enumerate several of their properties, prove Gödel’s $\beta$-Function Lemma, and demonstrate its first applications to coding techniques.

**Definition.** For $R\subseteq \omega^n$ a relation, $\chi_R : \omega^n \to \omega$, the *characteristic function* on $R$, is given by

$$ \chi_R = \begin{cases}
    1 & \text{if }\lnot R(\overline{a})\text{,} \\\\
    0 & \text{if }R(\overline{a})\text{.}
\end{cases} $$

**Definition.** A function from $\omega^m$ to $\omega$ ($m\geq 0$) is called **recursive** (or **computable**) if it is obtained by finitely many applications of the following rules: