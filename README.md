# CodeWars-Primes-in-numbers-5-kyu---Passed
Given a positive number n > 1 find the prime factor decomposition of n. 
 "(p1**n1)(p2**n2)...(pk**nk)"
with the p(i) in increasing order and n(i) empty if n(i) is 1.

Example: n = 86240 should return "(2**5)(5)(7**2)(11)"


TEST CASES:
fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(7919, "(7919)");
    testing(17*17*93*677, "(3)(17**2)(31)(677)");
    testing(933555431, "(7537)(123863)");
    testing(342217392, "(2**4)(3)(11)(43)(15073)");
    testing(35791357, "(7)(5113051)");
    testing(782611830, "(2)(3**2)(5)(7**2)(11)(13)(17)(73)");
    testing(775878912, "(2**8)(3**4)(17)(31)(71)");
    
    testing(79340, "(2**2)(5)(3967)");
    testing(841, "(29**2)");
    testing(18037, "(17)(1061)");
    testing(7572445, "(5)(1514489)");
    testing(860, "(2**2)(5)(43)");
    testing(943348, "(2**2)(7**2)(4813)");
    testing(73655, "(5)(14731)");
    testing(5725557, "(3**2)(29)(21937)");
    testing(28980, "(2**2)(3**2)(5)(7)(23)");
    testing(513, "(3**3)(19)");
    testing(181, "(181)");
    testing(358311, "(3)(83)(1439)");
    testing(7580, "(2**2)(5)(379)");
    testing(3803, "(3803)");
    testing(166, "(2)(83)");
    testing(77734, "(2)(38867)");
    testing(128, "(2**7)");
    testing(117, "(3**2)(13)");
    testing(112, "(2**4)(7)");
    testing(4673, "(4673)");
    testing(20135, "(5)(4027)");
    testing(66752, "(2**6)(7)(149)");
    testing(25490, "(2)(5)(2549)");
    testing(5253436, "(2**2)(1313359)");
    testing(188, "(2**2)(47)");
    testing(781006, "(2)(390503)");
    testing(599564, "(2**2)(7**3)(19)(23)");
    testing(127, "(127)");
    testing(181, "(181)");
    testing(942537, "(3)(211)(1489)");
    testing(938, "(2)(7)(67)");
    testing(647, "(647)");
    testing(182, "(2)(7)(13)");
    testing(512, "(2**9)");
    testing(119, "(7)(17)");
    testing(115, "(5)(23)");
    testing(114, "(2)(3)(19)");
    testing(183, "(3)(61)");
    testing(1314, "(2)(3**2)(73)");
    testing(3806, "(2)(11)(173)");
    testing(502, "(2)(251)");
    testing(966, "(2)(3)(7)(23)");
    testing(75822, "(2)(3)(12637)");
    testing(88792, "(2**3)(11)(1009)");
    testing(192, "(2**6)(3)");
    testing(119, "(7)(17)");
    testing(161, "(7)(23)");
    testing(63235, "(5)(12647)");
    testing(3548569, "(3548569)");
    testing(120, "(2**3)(3)(5)");    
    
}
