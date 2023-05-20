(def ack 
  (fn (m n)
    (cond ((= m 0) (+ n 1))
          ((= n 0) (ack (- m 1) 1))
          (:else (ack (- m 1) (ack m (- n 1)))))))

;; let ack m n = 
;;   if m = 0 then n + 1 
;;   elif n = 0 then ack (m - 1) 1 
;;   else ack (m - 1) (ack m (n - 1))
(let ack (m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        (:else (ack (- m 1) (ack m (- n 1))))))