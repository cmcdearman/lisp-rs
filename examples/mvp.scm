;; bindings are immutable by default
(def a 1)

;; mutable binding
(def! a 1)

;; if expression
;; Every if expression must have a then and an else clause.
;; The types of the then and else clause must be the same.
(if (= 1 2) 
    (println "1 is equal to 2")
    (println "1 is not equal to 2"))

;; lambda expression using `fn`
(fn (x) (+ x 1))

;; `defn` macro for defining functions
(defn (fib n)
  (let (loop n a b)
    (if (= n 0)
      a
      (loop (- n 1) b (+ a b)))
    (loop n 0 1)))

;; benchmark:
;; fib from 1 to 40
(def (main args)
  (let (loop n)
    (if (= n 40)
      ()
      (begin
        (println (fib n))
        (loop (+ n 1))))
    (loop 1)))

;; `defn` is a contraction for `def` and `fn`. The following are equivalent:
(defn f (x) (+ x 1))
(def f (fn (x) (+ x 1)))

;; `let` expression
(let ((a 1) 
      (b 2))
  (+ a b))

;; higher order function
(defn map (f xs)
  (if (empty? xs) ()
      (pair (f (head xs)) (map f (tail xs)))))

;; pattern matching
(defn map (f xs)
  (match xs
    ([])
    ((x :: xs) (pair (f x) (map f xs)))))

(def map f [] [])
(def map f (Pair x xs) (Pair (f x) (map f xs)))

(def fib 0 0)
(def fib 1 1)
(def fib n (+ (fib (- n 1)) (fib (- n 2))))

(defn fact (n)
  (if (= n 0)
      1
      (* n (fact (- n 1)))))

;; lists
'(1 2 3)
[1 2 3]

;; quasiquote/unquote
`(1 2 ,(+ 1 2))
[1 2 (+ 1 2)]

;; quasiquote/unquote-splicing
`(1 2 ,@(list 3 4))

;; vectors
#[1 2 3]

;; sets
#{ 1 2 3 }

;; maps
{ 'a 1 'b 2 }

;; maps bound
(def person { 'name "John" 'age 30 })
;; map access
(display person.name)
;; => "John"
(display person.age)
;; => 30

;; currying allows point-free style
(def sum (foldl + 0))

;; macros
(macro (cond clauses...)
  `(let (test (head clauses))
     (if (head test)
         (begin (tail test))
         (cond ,@(tail clauses)))))

(macro (when test body...)
  `(if ,test (begin ,@body) ()))

(macro (when test (vargs body))
  `(if ,test (begin ,@body) ()))

(macro (while test body...)
  `(let (loop)
     (if ,test
         (begin ,@body (loop))
         ([]))))

;; example uses
(while (< i 10)
  (println i)
  (set! i (+ i 1)))

(def (ack m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        ('t (ack (- m 1) (ack m (- n 1))))))

;; modules
(module List
  (def (pair head tail) { 'head head 'tail tail })
  (def empty '())

  (def (empty? xs)
    (= xs empty))

  (def (push-front xs x)
    (pair x xs))

  (def (map f xs)
    (if (empty? xs) ()
        (pair (f (head xs)) (map f (tail xs)))))
  
  (def (foldl f acc xs) 
    (if (empty? xs)
        acc
        (foldl f (f acc (head xs)) (tail xs))))
  
  (def (foldr f acc xs)
    (if (empty? xs)
        acc
        (f (head xs) (foldr f acc (tail xs)))))

  (def (filter f xs)
    (if (empty? xs)
        ()
        (if (f (head xs))
            (pair (head xs) (filter f (tail xs)))
            (filter f (tail xs)))))

  (def (rev xs)
    (foldl (fn (acc x) (pair x acc)) () xs)))

;; module usage
(use List)
(List.map (fn (x) (* x x)) [1 2 3])