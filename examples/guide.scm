;; global variables
(let a 10)
;; global constants
(const a 10)

;; `let` can bind to lambdas
(let gcd (lambda (a b)
  (if (= b 0)
    a
    (gcd b (% a b)))))

;; `let` can also bind application forms
(let (gcd a b)
  (if (= b 0)
      a
      (gcd b (% a b))))

;; you can use `match` to pattern match
(let (gcd a b)
  (match b
    (0 a)
    (_ (gcd b (% a b)))))

(let ((a 10) 
      (b 5))
  (gcd a b))

(struct point (x y))

(struct vec (arr len))

(let (fib n)
  (if ((<= n 1) n)
  (+ (fib (- n 1)) (fib (- n 2)))))

(println (fib 100000))