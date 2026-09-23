module Test.Main where

import Prelude

import Control.Monad.Except (runExcept)
import Data.Array as Array
import Data.Either (Either(..))
import Effect (Effect)
import Effect.Console (log)
import Foreign (F, MultipleErrors, isArray, isNull, isUndefined, readArray, readBoolean, readChar, readInt, readNumber, readString, tagOf, typeOf, unsafeFromForeign, unsafeToForeign)
import Foreign.Index (readIndex, readProp)
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

main :: Effect Unit
main = do
  log "Testing tags"

  assert $ typeOf (unsafeToForeign 1) == "number"
  assert $ tagOf (unsafeToForeign 1) == "Number"
  assert $ tagOf (unsafeToForeign 1.5) == "Number"
  assert $ tagOf (unsafeToForeign "x") == "String"
  assert $ tagOf (unsafeToForeign true) == "Boolean"
  assert $ tagOf (unsafeToForeign [ 1, 2 ]) == "Array"
  assert $ isArray (unsafeToForeign [ 1, 2 ])
  assert $ isNull (unsafeToForeign "x") == false
  assert $ isUndefined (unsafeToForeign unit)

  log "Testing readers"

  assertRight 3 (readInt (unsafeToForeign 3))
  assertRight 1.5 (readNumber (unsafeToForeign 1.5))
  assertRight "hello" (readString (unsafeToForeign "hello"))
  assertRight true (readBoolean (unsafeToForeign true))
  assertRight 'a' (readChar (unsafeToForeign 'a'))
  -- Single-character carriers must read back as one-character strings.
  assertRight "a" (readString (unsafeToForeign 'a'))
  assertLeft (readInt (unsafeToForeign "not an int"))
  assertLeft (readString (unsafeToForeign 12))
  assertLeft (readBoolean (unsafeToForeign 1))
  assertLeft (readChar (unsafeToForeign "toolong"))

  log "Testing unsafe coercions"

  assert $ unsafeFromForeign (unsafeToForeign "abc") == "abc"
  assert $ (unsafeFromForeign (unsafeToForeign 42) :: Int) == 42

  log "Testing arrays, properties and indexes"

  let array = unsafeToForeign [ 10, 20, 30 ]
  assertRight 3 (map Array.length (readArray array))
  assertRight 20 (readIndex 1 array >>= readInt)
  assertLeft (readIndex 9 array >>= readInt)

  let object = unsafeToForeign { name: "purescript", count: 2 }
  assertRight "purescript" (readProp "name" object >>= readString)
  assertRight 2 (readProp "count" object >>= readInt)
  assertLeft (readProp "missing" object >>= readInt)
  assertRight "purescript" (readProp "name" object >>= readString)
  assertRight [ "count", "name" ] (map Array.sort (Keys.keys object))

  log "Tests passed"
