module Main (main) where

import Coefs (b0, b1)

main :: IO ()
main = putStrLn ("y = " ++ show b1 ++ "x + " ++ show b0)