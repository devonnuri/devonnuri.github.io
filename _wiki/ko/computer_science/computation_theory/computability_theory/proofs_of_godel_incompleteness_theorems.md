---
title: 괴델의 불완전성 정리의 완전한 증명
created_at: 2023-06-28 19:29:45
updated_at: 2023-06-28 19:29:45
---
# 0단계 : 준비 사항

## 재귀적인 함수와 관계

재귀적이고 재귀적으로 열거 가능한 함수와 관계를 정의하고, 몇 가지 속성을 열거하고, 괴델의 $\beta$-함수 정리를 증명하고, 그것을 코딩 기법에 적용하는 방법을 살펴보자.

**정의.** 관계 $R\subseteq \omega^n$에 대해, $R$의 *특성 함수* $\chi_R : \omega^n \to \omega$는 다음과 같이 정의된다.

$$ \chi_R = \begin{cases}
    1 & \text{if }\lnot R(\overline{a})\text{,} \\\\
    0 & \text{if }R(\overline{a})\text{.}
\end{cases} $$

**정의.** $\omega^m$에서 $\omega$ ($m\geq 0$)로의 함수는 다음 규칙을 유한하게 반복 적용하여 얻을 수 있으면 **재귀적이다** (또는 **계산 가능하다**)고 정의한다.

* **R1**
    * $(x_1,\cdots,x_n)\mapsto x_i$에 의해 정의된 함수 $I_i^n : \omega^n\to\omega$, $1\leq i\leq n$는 *재귀적*이다.
    * 함수 $+:\omega\times\omega\to\omega$와 $\cdot : \omega\times\omega\to\omega$는 *재귀적*이다.
    * 함수 $\chi_< : \omega\times\omega\to\omega$는 *재귀적*이다.
* **R2** (합성)

    $H_i : \omega^n\to\omega$와 $G:\omega^k\to\omega$인 재귀적인 함수 $G$, $H_1$, $\cdots$, $H_k$에 대해 다음과 같이 정의되는 함수 $F:\omega^n\to\omega$는 *재귀적*이다.

    $$F(\overline{a})=G(H_1(\overline{a}),\cdots,H_k(\overline{a}))$$

* **R3** (최소화)

    재귀적인 함수 $G:\omega^{n+1}\to\omega$에서, 모든 $\overline{a}\in\omega^n$에 대해 $G(\overline{a},x)=0$인 $x\in\omega$가 존재할 때, 다음과 같이 정의된 $F:\omega^n\to\omega$는 *재귀적*이다.

    $$F(\overline{a})=\mu x$$

    (관계 $P$에 대해 $\mu x P(x)$은 $x\in P$가 얻을 수 있는 가장 작은 $x\in\omega$임을 기억하라.)

**정의.** $\chi_R$이 재귀적 함수라면, $R(\subseteq\omega^k)$은 **재귀적인** (또는 **계산 가능한**) **관계**라고 정의한다.

## 재귀적인 함수와 관계에 대한 성질

* **P0**

    $\sigma :\left\\{1,\cdots,k\right\\}\to\left\\{1,\cdots,n\right\\}$이 주어졌다 가정하자. $G:\omega^k\to\omega$가 재귀적이면, $\overline{a}=\left(a_1,\cdots,a_n\right)$에 대해 다음과 같이 정의된 함수 $F:\omega^n\to\omega$는 재귀적이다.

    $$F(\overline{a})=G\left(a_{\sigma(1)},\cdots,a_{\sigma(k)}\right)=G\left(I_{\sigma(1)}^n(\overline{a}),\cdots,I_{\sigma(k)}^n(\overline{a})\right)$$

    비슷하게, $P\left(x_1,\cdots,x_k\right)$가 재귀적이면,
    
    $$R\left(x_1,\cdots,x_n\right)\equiv P\left(a_{\sigma(1)},\cdots,a_{\sigma(k)}\right).$$

* **P1**

    재귀적인 관계 $Q\in\omega^k$와 재귀적인 함수 $H_1,\cdots,H_k : \omega^n\to\omega$에 대해, 

    $$P=\left\\{\overline{a}\in\omega^n | Q\left(H_1(\overline{a}),\cdots,H_k(\overline{a})\right)\right\\}$$

    는 재귀적 관계이다.

    *증명.* **R2**에 의해 $\chi_P(\overline{a})=\chi_Q\left(H_1(\overline{a}),\cdots,H_k(\overline{a})\right)$는 재귀적인 함수이다.

# 참고자료

1. Byunghan Kim, *Complete Proofs of Gödel's Incompleteness Theorems* (Lecture Note). [Link](https://web.yonsei.ac.kr/bkim/goedel.pdf)
