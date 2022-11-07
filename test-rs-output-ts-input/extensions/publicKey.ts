import { BinaryReader, BinaryWriter } from "borsh"
import { PublicKey } from "@solana/web3.js"

export const borshPublicKeyHack = () => {
	// "borsh": "^0.7.0"

	// agsol-borsh-schema/test-rs-output-ts-input/node_modules/borsh/lib/index.js:258
	//             writer[`write${capitalizeFirstLetter(fieldType)}`](value);
	//                                                               ^
	// TypeError: writer[capitalizeFirstLetter(...)] is not a function
  ;(BinaryReader.prototype as any).readPublicKeyHack = function () {
    const reader = this as unknown as BinaryReader
    const array = reader.readFixedArray(32)
    return new PublicKey(array)
  }
  ;(BinaryWriter.prototype as any).writePublicKeyHack = function (value: PublicKey) {
    const writer = this as unknown as BinaryWriter
    writer.writeFixedArray(value.toBytes())
  }
}
