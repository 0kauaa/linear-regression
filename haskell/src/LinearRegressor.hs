module LinearRegressor (b0, b1) where

import Functions ()
import Dataset   (x, y)

x2 :: [Double]
x2 = map (**2) x

xy :: [Double]
xy = zipWith (*) x y

n :: Double
n = fromIntegral (length x)

b1 :: Double
b1 = (n * sum xy - sum x * sum y) / (n * sum x2 - sum x ** 2)

b0 :: Double
b0 = (sum y - b1 * sum x) / n