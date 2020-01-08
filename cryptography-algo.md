# 基础数学知识点
## 模运算
已知正整数a，b，我们求出a除以b的商与余数：

```a * q + r = b```

其中q是商，r是余数，也可以记做：

```a mod b = r```，这里的mod即称为模运算，以上表达式读作“a模b等于r”或者“a模b余r”。

这里举几个举例：

```
9 mod 4 = 1
100 mod 7 = 2
```

所有mod b值相同的整数构成的集合，称为一个[剩余类](https://baike.baidu.com/item/%E5%89%A9%E4%BD%99%E7%B1%BB/3712708)。例如：集合```{3n+1}```即为mod 3的一个剩余类。对于整数n，其剩余类个数为n个，分别对应mod n余0、1、2...n-1。任何整数a在且仅在mod n的一个剩余类中。

所谓“[剩余系](https://baike.baidu.com/item/%E5%89%A9%E4%BD%99%E7%B3%BB)”，就是指对于某一个特定的正整数n，一个整数集中的数模n所得的余数域。例如：集合```{2, 5, 9}```模3的剩余系为：```{2, 2, 0}```。

从模n的每个剩余类中各取一个数，得到一个由n个数组成的集合，叫做模n的一个[完全剩余系](https://baike.baidu.com/item/%E5%AE%8C%E5%85%A8%E5%89%A9%E4%BD%99%E7%B3%BB)。例如，集合{3, 4, 5}为模3的一个完全剩余系。

和模运算相关的定理有蛮多的，例如：[威尔逊定理](https://baike.baidu.com/item/%E5%A8%81%E5%B0%94%E9%80%8A%E5%AE%9A%E7%90%86)，[费马小定理](https://baike.baidu.com/item/%E8%B4%B9%E9%A9%AC%E5%B0%8F%E5%AE%9A%E7%90%86)，[欧拉定理](https://baike.baidu.com/item/%E6%AC%A7%E6%8B%89%E5%AE%9A%E7%90%86)，感兴趣的同学可以移步仔细阅读体会。本文稍后会介绍***欧拉定理***。

## 最大公约数
最大公约数，是指能够被两个正整数m，n整除的最大的正整数，记做：```gcd(m, n)```

这里举几个举例：

```
gcd(2, 6) = 2
gcd(6, 39) = 3
```

## 辗转相除法

辗转相除法又称欧几里德算法，是指用于计算两个正整数m，n的最大公约数。应用领域有数学和计算机两个方面。计算公式：

```gcd(m,n) = gcd(n, m mod n)```

例如，我们要计算```gcd(62, 26)```

| 步骤 | 公式 | m mod n  |
| --- | --: | --:|
1 | gcd(62, 26) | 6 |
2 | gcd(26, 6) | 2 |
3 | gcd(6, 2) | 0 |

这样，gcd(62, 26) = 2

todo...............................................

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






























