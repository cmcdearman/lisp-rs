;;;; A file for feature/syntax ideas

;;; ==================================================================
;;; *                         Special Forms                          *
;;; ==================================================================

;; `let`
;; `if`
;; `lambda`
;; `quote`
;; `macro`

;; You can use `let` to bind values to names. Note
;; that we don't call these variables because they
;; can't vary. They are immutable.
(let x 10)

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

(let (map f xs)
  (if (empty? xs) nil
      (cons (f (head xs)) (map f (tail xs)))))

(let (fib n)
  (if (<= n 1) 
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(display (fib 45))

(let (gcd a b)
  (if (= b 0) 
      a
      (gcd b (% a b)))
  (gcd 10 5))

;;; ==================================================================
;;; *                            Macros                              *
;;; ==================================================================

;; `macro` is used to letine macros. Macros are
;; like functions, but they are evaluated at compile
;; time. They can be used to letine new syntax.

;; `begin`
(macro (begin . body)
  (if (null? body)
      nil
      (if (null? (tail body))
          (head body)
          `(let ((result ,(head body)))
             (if (null? result)
                 (begin . ,(tail body))
                 result
                 (begin . ,body))))))

;; `loop` is a macro that expands into a `let` that
;; binds a name to a lambda that calls itself.
(macro (loop . body)
  `(let loop ()
     (begin . ,body)
     (loop)))

;; `for` is a macro that expands into a `loop` that
;; binds a name to a range of numbers.
(macro (for i from to . body)
  `(loop (let i from)
     (if (<= i to)
         (begin . ,body)
         (inc i))))

(macro (backwards . body)
  (cons 'begin
	(reverse body)))

(macro (while condition . body)
  `(let loop ()
     (cond (,condition
	    (begin . ,body)
	    (loop)))))

(macro (when test . expr)
  (list 'if test (cons 'progn expr)))

;; you can use `match` built-in macro to pattern match or `if` for conditionals
(let (gcd a b)
  (match b
    ((0) a)
    ((_) (gcd b (% a b)))))

(let (fib n)
  (match n
    ((0) 0)
    ((1) 1)
    ((_) (+ (fib (- n 1)) (fib (- n 2))))))

(let (ack m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        (ack (- m 1) (ack m (- n 1)))))

(lets ((a 10) 
      (b 5))
  (+ a b))

;; Macro calls are like function calls, but the arguments
;; are not evaluated. Instead, they are passed to the macro
;; as unevaluated forms.

(when (= 1 1)
  (println "1 is equal to 1"))

;;; ==================================================================
;;; *                         Data Types                             *
;;; ==================================================================

;; Records with `data` are immutable product types with named fields.
(data (point x y))

;; Instantiating a record is just like calling a function.
(let p (point 1 2)) 

(class Eq ()
  (let (eq self other)
    (raise :NotImplementedError)))

(class Ord (Eq PartialOrd)
  (let (cmp self other)
    (raise :NotImplementedError)))

(class Stack ()
  (let (push self x)
    (raise :NotImplementedError))
  (let (pop self)
    (raise :NotImplementedError))
  (let (peek self)
    (raise :NotImplementedError))
  (let (empty? self)
    (raise :NotImplementedError)))

;; instantiate a class
(let s (Stack))

;; method calls are done with `send`
(send s push 1)