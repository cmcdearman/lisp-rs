
(fn ack (m n)
  (if 
    ((= m 0) (+ n 1))
    ((= n 0) (ack (- m 1) 1))
    (ack (- m 1) (ack m (- n 1)))))

;; let x = 1
(let x 1)

;; let x = 1 in x + 1
(let x 1 (+ x 1))

;; let (x, y) = (1, 2) in x + y
(let [x y] [1 2] (+ x y))