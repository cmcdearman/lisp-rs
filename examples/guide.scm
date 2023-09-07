;; You can use `let` to bind values to names. Note
;; that we don't call these variables because they
;; can't vary. They are immutable.
(let x 10)

;; `let` can bind to lambdas
(let gcd (lambda (a b)
  (if ((= b 0) a)
    (gcd b (% a b)))))

;; `let` can also bind application forms
(let (gcd a b)
  (if ((= b 0) a)
      (gcd b (% a b))))

;; you can use `match` to pattern match or `if` for conditionals
(let (gcd a b)
  (match b
    (0 a)
    (_ (gcd b (% a b)))))

(let (fib n)
  (match n
    (0 0)
    (1 1)
    (_ (+ (fib (- n 1)) (fib (- n 2))))))

(fn ack (m n)
  (if ((= m 0) (+ n 1))
      ((= n 0) (ack (- m 1) 1))
      (ack (- m 1) (ack m (- n 1)))))

(let (map f xs)
  (if (empty? xs) nil
      (cons (f (car xs)) (map f (cdr xs)))))

;; or you can match to declare functions
(let (gcd a 0) a)
(let (gcd a b) (gcd b (% a b)))

(let (fib 0) 0)
(let (fib 1) 1)
(let (fib n) (+ (fib (- n 1)) (fib (- n 2))))

(let (ack 0 n) (+ n 1))
(let (ack m 0) (ack (- m 1) 1))
(let (ack m n) (ack (- m 1) (ack m (- n 1))))

(let (map f ()) '())
(let (map f (x . xs)) (cons (f x) (map f xs)))
(let (map f (? (map? xs))) )

(let (fib n)
  (if ((<= n 1) n)
      (+ (fib (- n 1)) (fib (- n 2)))))

(println (fib 100000))

;; Macros
(lets ((a 10) 
      (b 5))
  (+ a b))

;; `struct` is a macro that defines a struct.
;; Under the hood, it's just a map. 
(struct point (x y))
