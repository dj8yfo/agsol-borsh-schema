import {
    RandomStruct,
    TestEnum,
    TestEnumVariantA,
    TestEnumVariantB,
    TestEnumVariantC,
    TestEnumVariantD,
    TestEnumVariantE,
    TestEnumVariantF,
    TestEnumVariantG,
    SCHEMA
} from "./schema";
import BN from "bn.js";
import { PublicKey } from '@velas/web3';

import { serialize, deserializeUnchecked } from "borsh";
import assert from "assert";
import fs from "fs";


var data = fs.readFileSync("../test-data/test_enums.json");
const enumData = JSON.parse(data.toString());

const testEnumVariantA = new TestEnumVariantA({});
const testEnumVariantB = new TestEnumVariantB({});
const testEnumVariantC = new TestEnumVariantC({
    unnamed_0: new BN(1234567890)
});
const testEnumVariantD = new TestEnumVariantD({
    unnamed_0: new PublicKey(12)
});
const testEnumVariantE = new TestEnumVariantE({
    unnamed_0: null
});
const testEnumVariantF = new TestEnumVariantF({
    unnamed_0: new RandomStruct({
        fieldA: "a test string",
        fieldB: Uint8Array.from([5, 6, 20, 21]),
    })
});
const testEnumVariantG = new TestEnumVariantG({
    hello: new Array(1, 2, 3, 4, 5),
    bello: new Array(new PublicKey(22), new PublicKey(23), new PublicKey(24)),
    yello: 234,
    zello: false,
});

const enumVariantA = new TestEnum({ testEnumVariantA });
const enumVariantB = new TestEnum({ testEnumVariantB });
const enumVariantC = new TestEnum({ testEnumVariantC });
const enumVariantD = new TestEnum({ testEnumVariantD });
const enumVariantE = new TestEnum({ testEnumVariantE });
const enumVariantF = new TestEnum({ testEnumVariantF });
const enumVariantG = new TestEnum({ testEnumVariantG });

const deEnumVariantA = deserializeUnchecked(
    SCHEMA,
    TestEnum,
    Buffer.from(enumData.enumVariantA)
);
const deEnumVariantB = deserializeUnchecked(
    SCHEMA,
    TestEnum,
    Buffer.from(enumData.enumVariantB)
);
const deEnumVariantC = deserializeUnchecked(
    SCHEMA,
    TestEnum,
    Buffer.from(enumData.enumVariantC)
);
const deEnumVariantD = deserializeUnchecked(
    SCHEMA,
    TestEnum,
    Buffer.from(enumData.enumVariantD)
);
const deEnumVariantE = deserializeUnchecked(
    SCHEMA,
    TestEnum,
    Buffer.from(enumData.enumVariantE)
);
const deEnumVariantF = deserializeUnchecked(
    SCHEMA,
    TestEnum,
    Buffer.from(enumData.enumVariantF)
);
const deEnumVariantG = deserializeUnchecked(
    SCHEMA,
    TestEnum,
    Buffer.from(enumData.enumVariantG)
);

export function testEnum1() {
	const serEnumVariantA = Array.from(serialize(SCHEMA, enumVariantA));
	assert(serEnumVariantA.length === enumData.enumVariantA.length);
	for (var i = 0; i < serEnumVariantA.length; i++) {
			assert(serEnumVariantA[i] === enumData.enumVariantA[i])
	}
	const serEnumVariantB = Array.from(serialize(SCHEMA, enumVariantB));
	assert(serEnumVariantB.length === enumData.enumVariantB.length);
	for (var i = 0; i < serEnumVariantB.length; i++) {
			assert(serEnumVariantB[i] === enumData.enumVariantB[i])
	}
	const serEnumVariantC = Array.from(serialize(SCHEMA, enumVariantC));
	assert(serEnumVariantC.length === enumData.enumVariantC.length);
	for (var i = 0; i < serEnumVariantC.length; i++) {
			assert(serEnumVariantC[i] === enumData.enumVariantC[i])
	}
	const serEnumVariantD = Array.from(serialize(SCHEMA, enumVariantD));
	assert(serEnumVariantD.length === enumData.enumVariantD.length);
	for (var i = 0; i < serEnumVariantD.length; i++) {
			assert(serEnumVariantD[i] === enumData.enumVariantD[i])
	}
	const serEnumVariantE = Array.from(serialize(SCHEMA, enumVariantE));
	assert(serEnumVariantE.length === enumData.enumVariantE.length);
	for (var i = 0; i < serEnumVariantE.length; i++) {
			assert(serEnumVariantE[i] === enumData.enumVariantE[i])
	}
	const serEnumVariantF = Array.from(serialize(SCHEMA, enumVariantF));
	assert(serEnumVariantF.length === enumData.enumVariantF.length);
	for (var i = 0; i < serEnumVariantF.length; i++) {
			assert(serEnumVariantF[i] === enumData.enumVariantF[i])
	}
	const serEnumVariantG = Array.from(serialize(SCHEMA, enumVariantG));
	assert(serEnumVariantG.length === enumData.enumVariantG.length);
	for (var i = 0; i < serEnumVariantG.length; i++) {
			assert(serEnumVariantG[i] === enumData.enumVariantG[i])
	}
}

export function testEnum2() {
	assert(deEnumVariantA.enum === enumVariantA.enum);
	assert(deEnumVariantB.enum === enumVariantB.enum);
	assert(deEnumVariantC.enum === enumVariantC.enum);
	assert(deEnumVariantD.enum === enumVariantD.enum);
	assert(deEnumVariantE.enum === enumVariantE.enum);
	assert(deEnumVariantF.enum === enumVariantF.enum);
	assert(deEnumVariantG.enum === enumVariantG.enum);

	assert(deEnumVariantC.testEnumVariantC.unnamed_0.toNumber()
			=== enumVariantC.testEnumVariantC.unnamed_0.toNumber());
	assert(deEnumVariantD.testEnumVariantD.unnamed_0.toString()
			=== enumVariantD.testEnumVariantD.unnamed_0.toString());
	assert(deEnumVariantE.testEnumVariantE.unnamed_0
			== enumVariantE.testEnumVariantE.unnamed_0);
	assert(deEnumVariantF.testEnumVariantF.unnamed_0.fieldA
			=== enumVariantF.testEnumVariantF.unnamed_0.fieldA);
	for (var i = 0; i < deEnumVariantF.testEnumVariantF.unnamed_0.fieldB.length; i++) {
			assert(deEnumVariantF.testEnumVariantF.unnamed_0.fieldB[i]
					=== enumVariantF.testEnumVariantF.unnamed_0.fieldB[i]);
	}

	for (var i = 0; i < deEnumVariantG.testEnumVariantG.hello.length; i++) {
			assert(deEnumVariantG.testEnumVariantG.hello[i]
					=== enumVariantG.testEnumVariantG.hello[i]);
	}
	for (var i = 0; i < deEnumVariantG.testEnumVariantG.bello.length; i++) {
			assert(deEnumVariantG.testEnumVariantG.bello[i].toString()
					=== enumVariantG.testEnumVariantG.bello[i].toString());
	}
	assert(deEnumVariantG.testEnumVariantG.yello === enumVariantG.testEnumVariantG.yello);
	assert(!deEnumVariantG.testEnumVariantG.zello);
}
