# 前言

本文分为上下两个篇幅，第一部分是基础数据知识，包括了：模运算、最大公约数、辗转相除法、扩展欧几里得算法、模运算的逆元、欧拉函数、欧拉定理等。

第二部分是RSA算法的介绍与代码部分，包括了：算法准备、数据加解密、RSA算法证明、蒙哥马利算法、RSA算法代码与示例等。

希望读者通过阅读此文，能够将RSA算法的来龙去脉搞清楚，同时可以基于RUST语言开发一个简单的RSA算法模拟。

---

# 基础数学知识点
## 模运算
已知正整数a，b，我们求出a除以b的商与余数：

```b * q + r = a```

其中q是商，r是余数，也可以记做：

```a ≡ r (mod b)```，这里的mod即称为模运算，以上表达式读作“a模b等于r”或者“a模b余r”。

这里举几个举例：

```
9 ≡ 1 (mod 4)
100 ≡ 2 (mod 7)
```

---

所有mod b值相同的整数构成的集合，称为一个[剩余类](https://baike.baidu.com/item/%E5%89%A9%E4%BD%99%E7%B1%BB/3712708)。例如：集合```{3n+1}```即为mod 3的一个剩余类。对于整数n，其剩余类个数为n个，分别对应mod n余0、1、2...n-1。任何整数a在且仅在mod n的一个剩余类中。

所谓“[剩余系](https://baike.baidu.com/item/%E5%89%A9%E4%BD%99%E7%B3%BB)”，就是指对于某一个特定的正整数n，一个整数集中的数模n所得的余数域。例如：集合```{2, 5, 9}```模3的剩余系为：```{2, 2, 0}```。

从模n的每个剩余类中各取一个数，得到一个由n个数组成的集合，叫做模n的一个[完全剩余系](https://baike.baidu.com/item/%E5%AE%8C%E5%85%A8%E5%89%A9%E4%BD%99%E7%B3%BB)。例如，集合{3, 4, 5}为模3的一个完全剩余系。

---

在与模n互素的全体剩余类中，从每一个类中各任取一个数作为代表组成的集合，叫做模m的一个简化剩余系，或称作缩系。例如，模5的一个简化剩余系是```{1，2，3，4}```，模10的一个简化剩余系是```{1，3，7，9}```，模18的一个简化剩余系是```{1，5，7，11，13，17}```。

和模运算相关的定理有蛮多的，例如：[威尔逊定理](https://baike.baidu.com/item/%E5%A8%81%E5%B0%94%E9%80%8A%E5%AE%9A%E7%90%86)，[费马小定理](https://baike.baidu.com/item/%E8%B4%B9%E9%A9%AC%E5%B0%8F%E5%AE%9A%E7%90%86)，[欧拉定理](https://baike.baidu.com/item/%E6%AC%A7%E6%8B%89%E5%AE%9A%E7%90%86)，感兴趣的同学可以移步仔细阅读体会。本文稍后会介绍***欧拉定理***。

---

## 最大公约数
最大公约数，是指能够被两个正整数m，n整除的最大的正整数，记做：```gcd(m, n)```

这里举几个举例：

```
gcd(2, 6) = 2
gcd(6, 39) = 3
```

---

## 辗转相除法

辗转相除法又称欧几里德算法，是指用于计算两个正整数m，n的最大公约数。应用领域有数学和计算机两个方面。计算公式：

```gcd(m,n) = gcd(n, m mod n)```

例如，我们要计算```gcd(62, 26)```

| 步骤 | 公式 | m mod n  |
| --- | --: | --:|
1 | gcd(62, 26) | 10 |
2 | gcd(26, 10) | 6 |
3 | gcd(10, 6) | 4 |
4 | gcd(6, 4) | 2 |
5 | gcd(4, 2) | 0 |

这样，gcd(62, 26) = 2

具体证明请参考[这里](https://baike.baidu.com/item/%E6%AC%A7%E5%87%A0%E9%87%8C%E5%BE%97%E7%AE%97%E6%B3%95)。

---

这里是辗转相除法的RUST实现代码，或者参考[这里](https://github.com/alexxuyang/cryptography-algo/blob/master/src/gcd.rs)。

```RUST
pub fn gcd(x: u64, y: u64) -> u64 {
    let remainder = x % y;

    if remainder == 0 {
        return y;
    } else {
        return gcd(y, remainder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(6, 27), 3);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(27, 6), 3);
    }
}
```

---

## 扩展欧几里得算法

扩展欧几里得算法（Extended Euclidean Algorithm，下面简称EEA）是欧几里得算法（又叫辗转相除法）的扩展。该算法基于这个一样假设，对于整数a，b，除了计算其gcd值外，我们是否可以找到整数x、y（其中一个很可能是负数），使得：ax + by = gcd(a, b)。

在欧几里得算法计算过程中，我们仅仅使用到了每次迭代的余数，EEA则使用到了商。下面我们使用一个例子，来看看具体的一个步骤：

假设：a = 240，b = 46

---

| 序号 | 商 Q<sub>i</sub> | 余数 R<sub>i</sub> | X<sub>i</sub> | Y<sub>i</sub> | Sum<sub>i</sub> |
| --- | --: | --: | --: | --: | --: |
1 | | 240 | 1 | 0 | 1 * 240 + 0 * 46 = 240 |
2 | | 46 | 0 | 1 | 0 * 240 + 1 * 46 = 46 |
3 | 240 / 46 = 5 | 240 % 46 = 10 | 1 - 5 * 0 = 1 | 0 - 5 * 1 = -5 | 1 * 240 + (-5) * 46 = 10 |
4 | 46 / 10 = 4 | 46 % 10 = 6 | 0 - 4 * 1 = -4 | 1- 4 * (-5) = 21 | (-4) * 240 + 21 * 46 = 6 |
5 | 10 / 6 = 1 | 10 % 6 = 4 | 1 - 1 * (-4) = 5 | (-5) - 1 * 21 = -26 | 5 * 240 + (-26) * 46 = 4 |
6 | 6 / 4 = 1 | 6 % 4 = 2 | (-4) - 1 * 5 = -9 | 21 - 1 * (-26) = 47 | (-9) * 240 + 47 * 46 = 2 |
7 | 4 / 2 = 2 | 4 % 2 = 0 | 5 - 2 * (-9) = 23 | (-26) - 2 * 47 = -120 | 23 * 240 + (-120) * 46 = 0 |

---

在上面的步骤中，如果只看左侧的三列，这就是辗转相除法的计算步骤。第七步算出余数R<sub>7</sub>是零，那么这两个数的gcd则为R<sub>6</sub> = 2。

这里，在右侧又多了三列，其中X<sub>i</sub>与Y<sub>i</sub>都使用到了商Q<sub>i</sub>, 其计算公式为：

X<sub>i</sub> = X<sub>i-2</sub> - Qi * X<sub>i-1</sub>

Y<sub>i</sub> = Y<sub>i-2</sub> - Qi * Y<sub>i-1</sub>

Sum<sub>i</sub> = X<sub>i</sub> * a + Y<sub>i</sub> * b

算法迭代的第一步，我们看第三行，X<sub>3</sub> = 1, Y<sub>3</sub> = -5, 代入公式算出：Sum<sub>3</sub> = 10 = R<sub>3</sub>

---

根据X<sub>i</sub>与Y<sub>i</sub>的计算公式，不难得出每一步中，Sum<sub>i</sub> = R<sub>i</sub>。具体推演过程中也验证了这个想法。

这样，根据这个迭代的过程，到了第7步，我们算出了gcd是2，那么反推到第6步算出的X<sub>6</sub>与Y<sub>6</sub>，即是我们需要的解，使得：```(-9) * 240 + 47 * 46 = 2```。

EEA的具体证明请参考[这里](https://baike.baidu.com/item/%E6%89%A9%E5%B1%95%E6%AC%A7%E5%87%A0%E9%87%8C%E5%BE%97%E7%AE%97%E6%B3%95)。

好了，说了这么多，这个EEA究竟用来干啥用？？？

好吧，且听我们下一节分解哈:smirk::smirk::smirk:

---

## 模运算的逆元

逆元是抽象代数中的一个概念，指在群G中任意一个元素a，都在G中有唯一的逆元a'，具有性质：

```a · a' = a' · a = e``` ( · 为该群中定义的运算)。其中，e为该群的单位元。

将这个概念类推到模运算中，模运算单位元是 1。

那么对于整数n来说，逆元的定义为，对于任意a，存在a'，使得：

```a * a' ≡ 1 (mod n)```

---

定义逆元函数：

```inv(a, n) = a'```

我们举几个例子：

```
inv(5, 11) = 9
inv(7, 13) = 2
inv(10, 31) = 28
inv(12, 29) = 17
```

---

定理：假设p为质数，对于任何正整数a，a < p，那么存在唯一的逆元a'，a' < p，使得：

```a * a' ≡ 1 (mod p)```

使用反证法可以证明该定理，同学们自己开一下脑洞。

假设已知一个大质数p，例如质数10888869450418352160768000001，我们又知道a，怎么求出a的逆元呢？我们可以穷举，从2开始，一直到p-2，计算a * a' mod p的值，直到等于1为止。

---

那么，是否还有更好的方法呢？有，方法之一就是使用EEA算法。

因为p是质数，对于任意a < p，gcd(a, p) = 1

又根据EEA，我们知道可以求得：x和y，使得：a * x + p * y = gcd(a, p) = 1

所以：**a * x ≡ 1 (mod p)**

**所以a的逆元就是x**。这里需要注意，有可能此方法算出的x是负数，或者大小超过了p，所以最后需要对x做一个优化。详见代码。

---

这里是EEA和INV的RUST实现代码，或者参考[这里](https://github.com/alexxuyang/cryptography-algo/blob/master/src/ext_euclid.rs)。

```RUST
fn ext_euclid(x: i64, y: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    if y == 0 {
        return (1, 0, x);
    } else {
        while r != 0 {
            let q = old_r / r;

            let t_r = r;
            r = old_r - q * r;
            old_r = t_r;

            let t_s = s;
            s = old_s - q * s;
            old_s = t_s;

            let t_t = t;
            t = old_t - q * t;
            old_t = t_t;
        }
        return (old_s, old_t, old_r);
    }
}

fn inv(a: i64, p: i64) -> i64 {
    let (r, _, _) = ext_euclid(a, p);
    return ((r % p) + p) % p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext_euclid_works() {
        assert_eq!(inv(5, 11), 9);
        assert_eq!(inv(7, 13), 2);
        assert_eq!(inv(10, 31), 28);
        assert_eq!(inv(12, 29), 17);
    }
}
```

---

## 欧拉函数

对正整数n，欧拉函数是小于或等于n的正整数中与n互质的数的数目（因此φ(1) = 1）。此函数以其首名研究者欧拉命名(Euler's totient function)，它又称为Euler's totient function、φ函数、欧拉商数等。 例如φ(8) = 4，因为1,3,5,7均和8互质。

欧拉函数如下所示：

![](http://latex.codecogs.com/gif.latex?\\varphi(x)=x\\prod_{i=1}^{n}\\left(1-\\frac{1}{p_{i}}\\right))

其中，x有n个质因数，p<sub>i</sub>是第i个质因数。例如我们需要计算φ(8)，8的质因数只有2，那么：

φ(8) = 8 * (1 - 1/2) = 8 * 1/2 = 4，正确

对于任何质数p，φ(p) = p - 1

对于任何质数p和q(p != q)，φ(p * q) = (p - 1) * (q - 1)

---

## 欧拉定理

欧拉定理（Euler Theorem，也称费马-欧拉定理或欧拉函数定理）是一个关于同余的性质。欧拉定理得名于瑞士数学家莱昂哈德·欧拉，该定理被认为是数学世界中最美妙的定理之一。欧拉定理实际上是费马小定理的推广。

欧拉定理表明，若n,a为正整数，且n,a互质，则：

![](https://latex.codecogs.com/gif.latex?a^{\\varphi(n)}&space;\\equiv&space;1(\\bmod&space;n))

其中，φ(n)是欧拉函数。

---

这里做一个简洁的证明：

将1-n中，与n互质的数，定义为集合A：{X1, X2 ... , X<sub>φ(n)</sub>}，可知总共有φ(n)个数。同时它们都与n互质。所以集合A是mod n的一个**缩系**。

构造集合B：{Y1 = a * X1, Y2 = a * X2 ... , Y<sub>φ(n)</sub> = a * X<sub>φ(n)</sub>}，因为a、X<sub>i</sub>都与n互质，可知Y<sub>i</sub>也都与n互质。

这样，集合B是否也是一个**缩系**？假设不是一个缩系，那么必存在i、j，使得：Y<sub>i</sub> ≡ Y<sub>j</sub> (mod n)

可得：a * X<sub>i</sub> - a * X<sub>j</sub> ≡ 0 (mod n) => a * (X<sub>i</sub> - X<sub>j</sub>) ≡ 0 (mod n)

由于a与n互质，可知：X<sub>i</sub> - X<sub>j</sub> ≡ 0 (mod n)，显然，这与X<sub>i</sub>的定义相矛盾，X<sub>i</sub>、X<sub>j</sub>是小于n的不同整数。

所以，集合B也是一个**缩系**。

---

这样，我们有两个mod n的缩系集合A、B，根据缩系的定义，将两个集合的各自元素求其积，可知这两个积mod n是相等的：

(a * X1) * (a * X2) * (a * X<sub>φ(n)</sub>) ≡ X1* X2 * X<sub>φ(n)</sub> mod n

=> a<sup>φ(n)</sup> * (X1X2...X<sub>φ(n)</sub>) ≡ X1X2...X<sub>φ(n)</sub> mod n

=> (a<sup>φ(n)</sup> - 1) * (X1X2...X<sub>φ(n)</sub>) ≡ 0 mod n

由于X<sub>i</sub>与n互质，可得：a<sup>φ(n)</sup> - 1 ≡ 0 mod n => a<sup>φ(n)</sup> ≡ 1 mod n，定理得证。

---

# RSA算法

## 综述

终于讲到RSA算法了，大家都还跟住的吧:joy::joy:

RSA算法是由Ron Rivest、Ad Shamir、Leonard Adleman，于1977年提出的，RSA就是他们三位名字的首字母拼在一起组成的。

RSA是一种非对称加密算法，其核心思路是，alice公开自己的公钥K<sub>pub</sub>，同时仅仅只有alice知道自己的私钥K<sub>pri</sub>；bob使用K<sub>pub</sub>对数据m进行加密，得到秘文c，并将c传输给alice。

此时，alice使用私钥K<sub>pri</sub>对密文c进行解密，得到明文数据m。由于只有alice知道私钥K<sub>pri</sub>，就算其他人得到了密文c，也无法解密出其明文来。对大整数进行因数分解的难度，是RSA算法安全性的保证。

---

## 算法准备

1. 选取两个质数p、q，p != q，计算其乘积：N = p * q
2. 计算：φ(N) = (p - 1) * (q - 1)
3. 让：r = φ(N)
3. 任选一整数e，1 < e < r，且：gcd(e, r) = 1
4. 根据算法EEA，求得e的逆元d，使得：ed ≡ 1 (mod r)
5. 最终，(N, e）即是公钥，（N, d）即是私钥
6. alice公开公钥（N，e），同时将私钥（N，d）自己私密保管

---

## 数据加密

bob对于任何消息m，将其转化成整数n，计算：c ≡ n<sup>e</sup> (mod N)，这里c即是密文。

## 数据解密

alice拿到密文，计算：n ≡ c<sup>d</sup> (mod N)，这里n即是明文。

---

## 算法证明

因为：ed ≡ 1 (mod r)

所以：ed = rm + 1，m为正整数

所以：c<sup>d</sup> ≡ n<sup>ed</sup> ≡ n<sup>rm + 1</sup> ≡ n * n<sup>rm</sup> (mod N)

根据欧拉定理，可知：n<sup>r</sup> ≡ 1 (mod N)，代入上式：

n * 1<sup>m</sup> ≡ n (mod N)

所以结论为：c<sup>d</sup> ≡ n (mod N)

---

## 蒙哥马利算法

在RSA算法的实现过程中，首先需要随机生成两个大质数，目前推荐至少是2048位的质数，这样可以保证安全。我们在程序模拟过程中，不会使用这么大的质数，仅仅做一个演示。

RSA需要的几个外部算法，gcd与EEA，在之前的部分都已经介绍过。可以注意到，其加密、解密的过程，牵涉到模运算与指数计算，因为真实环境下，e或者d这两个值，可能会非常的大（例如超过2<sup>1000</sup>），所以这里还需要有一些优化，不能直接计算n<sup>e</sup>、c<sup>d</sup>。

所以，这里还需要介绍一个算法 - 蒙哥马利算法，蒙哥马利(Montgomery)幂模运算是快速计算a<sup>b</sup> mod p的一种算法，是RSA加密算法的核心之一。需要注意的是，该算法仅仅是作为提升效率而使用，与RSA算法的安全性本身没有关系。

---

我们可以看一个简单的例子，例如需要计算A<sup>13</sup> mod P，我们可以这样：

| 步骤 | 公式 | 值 | 操作 | op_code |
| --- | --: | --:| --:| --:|
1 | A1 = 1 * 1 mod p | 1 | 二次方 | 1 |
2 | A2 = A1 * A mod p | A<sup>1</sup>  | 乘A | - |
3 | A3 = A2 * A2 mod p | A<sup>2</sup> | 二次方 | 1 |
4 | A4 = A3 * A mod p | A<sup>3</sup>  | 乘A | - |
5 | A5 = A4 * A4 mod p | A<sup>6</sup> | 二次方 | 0 |
6 | A6 = A5 * A5 mod p | A<sup>12</sup> | 二次方 | 1 |
7 | A7 = A6 * A mod p | A<sup>13</sup> | 乘A | - |

---

我们将13按照2进制显示为：0b1101，大家可以观察到，这里的四个数字，1101，和上表中的op_code的1101相对应，其意思是：

- 首先设置结果的初始值为1
- 将幂值的二进制表达式从左向右做循环
	- 遇到1，则做两个操作：二次方操作 + 乘A操作
	- 遇到0，则仅仅做：二次方操作

二次方操作，其实就是自乘，所以其核心思路是，将模幂运算降为了模乘运算，算法难度从O(2<sup>n</sup>)降为了O(n)。

---

## RSA算法代码与示例

至此所有的准备工作都好了，我们可以愉快的编码了 :joy::joy:

### 初始化

RSA算法初始化的部分如下，或者看[这里](https://github.com/alexxuyang/cryptography-algo/blob/master/src/bin/rsa_init.rs)。

---

```RUST
extern crate cryptography_algo;

use cryptography_algo::gcd;
use cryptography_algo::ext_euclid;

use rand::Rng;
use std::env;

fn main() {
    let p;
    let q; 

    if env::args().len() == 3 {

        p = i64::from_str_radix(&env::args().nth(1).unwrap(), 10).unwrap();
        q = i64::from_str_radix(&env::args().nth(2).unwrap(), 10).unwrap();

        if !prime_tools::is_u64_prime(p as u64) {
            panic!("p is not a prime number!");
        }

        if !prime_tools::is_u64_prime(q as u64) {
            panic!("q is not a prime number!");
        }
    } else {
        p = 2134324421;
        q = 1990843139;
    }

    let N: i64 = p * q;
    let r = (p - 1) * (q - 1);

    println!("p: {} ,q: {} N: {}, r: {}", p, q, N, r);

    let mut rng = rand::thread_rng();

    let mut e;
    loop {
        e = rng.gen::<u16>() as i64;
        if gcd::gcd(e, r)  == 1 {
            break;
        }
    }

    let d = ext_euclid::inv(e, r);

    println!("e: {}, d: {}", e, d);

    println!("public key is(N, e): ({}, {})", N, e);
    println!("private key is(N, d): ({}, {})", N, d);
}
```
核心部分很简单，根据p、q，计算N、r；随机生成e，需要gcd(e, r) = 1，同时求得e的逆元d。

---

### 加密解密

下面的是蒙哥马利算法的RUST代码，或者看[这里](https://github.com/alexxuyang/cryptography-algo/blob/master/src/power_mod.rs)。

```RUST
pub fn power_mod(base: i64, mut power: i64, N: i64) -> i64 {
    let mut bits = Vec::new();

    while power != 0 {
        match power & 1 {
           1 => bits.push(true),
           0 => bits.push(false),
           _ => {}
        }
        power = power >> 1;
    }

    let mut result: i64 = 1;
    while let Some(bit) = bits.pop() {
        result = mod_multiply(result, result, N);
        if bit {
            result = mod_multiply(result, base, N);
        }
    }

    result
}

pub fn mod_multiply(a: i64, b:i64, N: i64) -> i64 {
    ((a as i128 * b as i128) % (N as i128)) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_multiply_works() {
        assert_eq!(mod_multiply(10, 20, 17), 13);
        assert_eq!(mod_multiply(6, 27, 5), 2);
        assert_eq!(mod_multiply(4, 2, 19), 8);
        assert_eq!(mod_multiply(27, 6, 13), 6);
    }

    #[test]
    fn power_mod_works() {
        assert_eq!(power_mod(3, 4, 5), 1);
        assert_eq!(power_mod(6, 10, 5), 1);
        assert_eq!(power_mod(5, 13, 19), 17);
        assert_eq!(power_mod(27, 6, 51), 9);
        assert_eq!(power_mod(45, 13, 19), 7);
        assert_eq!(power_mod(1234567, 100, 199), 29);
        assert_eq!(power_mod(66887799, 1000000, 1001), 22);
        assert_eq!(power_mod(1357924680, 999999, 666889), 355775);
        assert_eq!(power_mod(113355778866, 99999, 981287), 797582);
        assert_eq!(power_mod(113355778866, 999999, 5050404053), 31958690);
    }
}
```

---

加解密算法入口程序如下，或者看[这里](https://github.com/alexxuyang/cryptography-algo/blob/master/src/bin/rsa_enc_dec.rs)。

```RUST
extern crate cryptography_algo;

use cryptography_algo::power_mod;

use std::env;

fn main() {
    if env::args().len() < 3 {
        println!("program should be start with: rsa_enc_dec base power modula");
        return;
    }

    let base = i64::from_str_radix(&env::args().nth(1).unwrap(), 10).unwrap();
    let power = i64::from_str_radix(&env::args().nth(2).unwrap(), 10).unwrap();
    let modula = i64::from_str_radix(&env::args().nth(3).unwrap(), 10).unwrap();

    println!("base: {}, power: {}, modula: {}", base, power, modula);

    println!("result: {}", power_mod::power_mod(base, power, modula));
}
```

---

### 运行示例

可以使用openssl命令得到质数，命令如下：

```
openssl prime -generate -bits 32 -safe
```

***算法初始化：***

```
target/release/rsa_init 2134324421 1990843139
p: 2134324421 ,q: 1990843139 N: 4249105129947997519, r: 4249105125822829960
e: 1343, d: 1920481616808978247
public key is(N, e): (4249105129947997519, 1343)
private key is(N, d): (4249105129947997519, 1920481616808978247)
```

初始化需要两个质数参数p、q（不输入则使用默认值），这里注意，由于程序限制p、q的值需要小于：sqrt(2<sup>63</sup>)，p、q最大为3037000493。

初始化的结果是，得到了公钥（N，e），与私钥（N，d）。这时，alice将公钥给到bob，私钥自己私密保管。

---

***数据加密：***

```
target/release/rsa_enc_dec 50412164937805327 1343 4249105129947997519
base: 50412164937805327, power: 1343, modula: 4249105129947997519
result: 1099769683952491905
```
bob拿到alice的公钥后，对自己的数据，进行加密计算。

该程序有三个参数，第一个是数据明文[50412164937805327]，后面两个参数分别为公钥中的e与N。

加密过程使用蒙哥马利算法进行模幂计算，最终得到了加密后的密文。

这时，bob将密文[1099769683952491905]给到alice。

---

***数据解密：***

```
target/release/rsa_enc_dec 1099769683952491905 1920481616808978247 4249105129947997519
base: 1099769683952491905, power: 1920481616808978247, modula: 4249105129947997519
result: 50412164937805327
```
alice拿到bob的密文数据后，结合自己的私钥数据，进行数据解密。

该程序有三个参数，第一个是密文数据[1099769683952491905]，后面两个参数分别为私钥中的d与N。

解密过程使用蒙哥马利算法进行模幂计算，最终得到了解密后的明文[50412164937805327]。

可以看到，最终解密出的明文数据，与最初bob生成的数据是一致的，bingo :smiley::smiley::smiley:
