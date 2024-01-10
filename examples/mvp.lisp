;; let binding
(let a 1)

;; if expression
;; Every if expression must have a then and an else clause.
;; The types of the then and else clause must be the same.
(if (= 1 2) 
    (println "1 is equal to 2")
    (println "1 is not equal to 2"))

;; let binding to function
(let (fib n)
  (let (loop n a b)
    (if (= n 0)
      a
      (loop (- n 1) b (+ a b)))
    (loop n 0 1)))

;; let binding to function with expression body
(let (gcd a b)
  (if (= b 0) 
      a
      (gcd b (mod a b)))
  (gcd 10 5))

(let (map f xs)
  (if (empty? xs) ()
      (pair (f (head xs)) (map f (tail xs)))))

(let (fact n)
  (if (= n 0)
      1
      (* n (fact (- n 1)))))

(let (ack m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        (t (ack (- m 1) (ack m (- n 1))))))

'(1 2 3)
`(1 2 ,(+ 1 2))
`(1 2 ,@(list 3 4))
[1 2 3]
#{ 1 2 3 }
{ :a 1 :b 2 }

;; macros
(macro (fn name args body)
  `(def ,name (lambda ,args) ,@body))

(macro (while test body)
  `(fn loop ()
     (if ,test
         (begin ,@body (loop))
         ())))