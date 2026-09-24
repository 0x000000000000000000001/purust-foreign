module Test.Main where

import Prelude

import Control.Monad.Except (Except, runExcept)
import Data.Array as Array
import Data.Either (Either(..), isLeft)
import Effect (Effect)
import Effect.Console (log)
import Foreign (F, Foreign, MultipleErrors, isArray, isNull, isUndefined, readArray, readBoolean, readChar, readInt, readNumber, readString, tagOf, typeOf, unsafeFromForeign, unsafeToForeign)
import Foreign.Index (class Index, hasOwnProperty, hasProperty, readIndex, readProp)
import Foreign.Keys as Keys
import Test.Assert (assert)

runF :: forall a. F a -> Either MultipleErrors a
runF = runExcept

assertRight :: forall a. Eq a => Show a => a -> F a -> Effect Unit
assertRight expected result = case runF result of
  Right actual -> assert (actual == expected)
  Left errors -> do
    log ("unexpected failure: " <> show errors)
    assert false

assertLeft :: forall a. Show a => F a -> Effect Unit
assertLeft result = case runF result of
  Left _ -> pure unit
  Right actual -> do
    log ("expected a failure but got " <> show actual)
    assert false

-- `hasProperty`/`hasOwnProperty` carry the `Index i m` fundep, so the call
-- site has to fix `m`; use the same concrete `Except` monad as the readers.
hasPropertyF :: forall i. Index i (Except MultipleErrors) => i -> Foreign -> Boolean
hasPropertyF = hasProperty

hasOwnPropertyF :: forall i. Index i (Except MultipleErrors) => i -> Foreign -> Boolean
hasOwnPropertyF = hasOwnProperty

main :: Effect Unit
main = do
  log "Testing tags"

  assert $ typeOf (unsafeToForeign 1) == "number"
  assert $ tagOf (unsafeToForeign 1) == "Number"
  assert $ tagOf (unsafeToForeign 1.5) == "Number"
  assert $ tagOf (unsafeToForeign "x") == "String"
  assert $ tagOf (unsafeToForeign true) == "Boolean"
  assert $ tagOf (unsafeToForeign [ 1, 2 ]) == "Array"
  assert $ typeOf (unsafeToForeign [ 1, 2, 3 ]) == "object"

  let identity :: Int -> Int
      identity x = x
  let add :: Int -> Int -> Int
      add x y = x + y
  assert $ typeOf (unsafeToForeign identity) == "function"
  assert $ tagOf (unsafeToForeign identity) == "Function"
  -- A partially applied closure keeps the function classification.
  assert $ typeOf (unsafeToForeign (add 1)) == "function"
  assert $ tagOf (unsafeToForeign (add 1)) == "Function"
  assert $ typeOf (unsafeToForeign { a: 1 }) == "object"
  assert $ tagOf (unsafeToForeign { a: 1 }) == "Object"

  assert $ isArray (unsafeToForeign [ 1, 2 ])
  assert $ isArray (unsafeToForeign [ 1, 2, 3 ]) == true
  assert $ isArray (unsafeToForeign 42) == false
  assert $ isArray (unsafeToForeign { a: 1 }) == false
  assert $ isArray (unsafeToForeign "test") == false

  assert $ isNull (unsafeToForeign "x") == false
  assert $ isNull (unsafeToForeign { a: 1 }) == false
  assert $ isUndefined (unsafeToForeign unit)
  assert $ isUndefined (unsafeToForeign "hello") == false
  assert $ isUndefined (unsafeToForeign { a: 1 }) == false

  log "Testing readers"

  assertRight 3 (readInt (unsafeToForeign 3))
  assertRight 42 (readInt (unsafeToForeign 42))
  assertRight 42 (readInt (unsafeToForeign 42.0))
  assertLeft (readInt (unsafeToForeign 42.5))
  assertRight 1.5 (readNumber (unsafeToForeign 1.5))
  assertRight 42.0 (readNumber (unsafeToForeign 42))
  assertRight (-42.0) (readNumber (unsafeToForeign (-42)))
  assertLeft (readNumber (unsafeToForeign "42"))
  assertRight "hello" (readString (unsafeToForeign "hello"))
  assertRight true (readBoolean (unsafeToForeign true))
  assertRight 'a' (readChar (unsafeToForeign 'a'))
  -- Single-character carriers must read back as one-character strings.
  assertRight "a" (readString (unsafeToForeign 'a'))
  assertLeft (readInt (unsafeToForeign "not an int"))
  assertLeft (readString (unsafeToForeign 12))
  assertLeft (readString (unsafeToForeign 42))
  assertLeft (readBoolean (unsafeToForeign 1))
  assertLeft (readBoolean (unsafeToForeign 42))
  assertLeft (readChar (unsafeToForeign "toolong"))

  log "Testing unsafe coercions"

  assert $ unsafeFromForeign (unsafeToForeign "abc") == "abc"
  assert $ (unsafeFromForeign (unsafeToForeign 42) :: Int) == 42

  log "Testing arrays, properties and indexes"

  let array = unsafeToForeign [ 10, 20, 30 ]
  assertRight 3 (map Array.length (readArray array))
  assert $ isLeft (runF (readArray (unsafeToForeign 42)))
  assertRight 20 (readIndex 1 array >>= readInt)
  assertRight 30 (readIndex 2 array >>= readInt)
  assertLeft (readIndex 9 array >>= readInt)

  let object = unsafeToForeign { name: "purescript", count: 2 }
  assertRight "purescript" (readProp "name" object >>= readString)
  assertRight 2 (readProp "count" object >>= readInt)
  assertLeft (readProp "missing" object >>= readInt)
  assertRight [ "count", "name" ] (map Array.sort (Keys.keys object))

  log "Testing hasProperty and hasOwnProperty"

  assert $ hasPropertyF "name" object
  assert $ hasOwnPropertyF "name" object
  assert $ not $ hasPropertyF "missing" object
  assert $ not $ hasOwnPropertyF "missing" object
  assert $ not $ hasOwnPropertyF "toString" object
  assert $ hasPropertyF "length" array
  assert $ hasOwnPropertyF "length" array
  assert $ hasPropertyF 1 array
  assert $ not $ hasPropertyF 9 array
  assert $ not $ hasOwnPropertyF 3 array
  assert $ not $ hasPropertyF "name" (unsafeToForeign 42)
  assert $ not $ hasOwnPropertyF "name" (unsafeToForeign 42)
  assert $ not $ hasPropertyF "name" (unsafeToForeign unit)

  log "Tests passed"
