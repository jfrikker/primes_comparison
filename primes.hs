import Control.Monad (forM_, when)
import System.Environment as Env

primes :: (Integral a) => [a]
primes = let x = 2 : filter (isPrime x) [3..]
         in x

isPrime :: (Integral a) => [a] -> a -> Bool
isPrime ps n = null factors
  where candidates = takeWhile (\i -> i * i <= n) ps
        factors = filter (\i -> n `rem` i == 0) candidates

main :: IO ()
main = do
  args <- Env.getArgs
  let n = read $ head args :: Int
  let p = primes
  forM_ [2..n] $ \i -> 
    when (isPrime p i) $ print i
