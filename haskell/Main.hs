{-# LANGUAGE ForeignFunctionInterface #-}

module Main where

import Foreign
import Foreign.C
import System.Win32.DLL

foreign import ccall "dynamic"
  mkFun :: FunPtr (IO ()) -> IO ()

main :: IO ()
main = do
  -- load lib (use https://downloads.haskell.org/~ghc/7.6.3/docs/html/libraries/unix-2.6.0.1/System-Posix-DynamicLinker.html for Linux support)
  lib <- loadLibrary "rapl_lib.dll"

  -- load start rapl
  startRaplPtr <- getProcAddress lib "start_rapl"
  let startRaplPtr' = castPtrToFunPtr startRaplPtr
  let startRaplFunction = mkFun startRaplPtr'

  -- load stop rapl
  stopRaplPtr <- getProcAddress lib "stop_rapl"
  let stopRaplPtr' = castPtrToFunPtr stopRaplPtr
  let stopRaplFunction = mkFun stopRaplPtr'

  -- benchmark
  startRaplFunction
  let loop n 0 = return n
      loop n i = do
        loop (n + 1) (i - 1)
  result <- loop 0 10000
  print result
  stopRaplFunction
