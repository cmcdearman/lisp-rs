;; let x = 1
(let x 1)

;; let x = 1 in x + 1
(let x 1 (+ x 1))

;; let (x, y) = (1, 2) in x + y
(let [x y] [1 2] (+ x y))