(define (ack m n)
  (cond ((eq m 0) (+ n 1))
        ((eq n 0) (ack (- m 1) 1))
        (else (ack (- m 1) (ack m (- n 1))))))

;; fn ack m n =
;;  if m == 0 then n + 1
;;  else if n == 0 then ack (m - 1) 1
;;  else ack (m - 1) (ack m (n - 1))
(fn ack (m n)
  (cond ((eq m 0) (+ n 1))
        ((eq n 0) (ack (- m 1) 1))
        (else (ack (- m 1) (ack m (- n 1))))))

(fn ack (m n)
  (if ((= m 0) (+ n 1))
  ((= n 0) (ack (- m 1) 1))
  (ack (- m 1) (ack m (- n 1)))))

;; let x = 1
(let x 1)

;; let x = 1 in x + 1
(let x 1 (+ x 1))

;; let x = 1 in let y = 2 in x + y
(let [x y] [1 2] (+ x y))
