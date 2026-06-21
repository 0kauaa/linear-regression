module Functions (mean, stddev) where

mean :: [Double] -> Double
mean xs = sum xs / fromIntegral (length xs)

stddev :: [Double] -> Double
stddev xs = sqrt $ (sum xs - mean xs) ^ 2 / fromIntegral (length xs - 1)