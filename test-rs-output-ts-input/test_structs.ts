import {
    OtherState,
    TestStruct,
    TupleStruct,
    SCHEMA
} from "./schema";

import BN from "bn.js";
import { serialize, deserializeUnchecked } from "borsh";
import assert from "assert";
import fs from "fs";

var data = fs.readFileSync("../test-data/test_structs.json");
const structData = JSON.parse(data.toString());


let otherStateOne = new OtherState({
    amount: new BN(1000000000),
    timestamp: new BN(1234567890),
});

let otherStateTwo = new OtherState({
    amount: new BN(2000000000),
    timestamp: new BN(1234567891),
});

let otherStateThree = new OtherState({
    amount: new BN(3000000000),
    timestamp: new BN(1234567892),
});

let testStructNone = new TestStruct(
    {
        fieldA: new BN(45678910),
        fieldB: 103,
        fieldC: null,
    }
);

let testStructSome = new TestStruct(
    {
        fieldA: new BN(10),
        fieldB: 113,
        fieldC: [otherStateOne, otherStateTwo, otherStateThree],
    }
);

let tupleStruct = new TupleStruct(
    {
        unnamed_0: 10,
        unnamed_1: -2,
        unnamed_2: otherStateOne,
    }
);

export function testStruct1() {

	// test struct with some field
	const deStructSome = deserializeUnchecked(SCHEMA, TestStruct, Buffer.from(structData.testStructSome));
	assert(deStructSome.fieldA.toNumber() === testStructSome.fieldA.toNumber());
	assert(deStructSome.fieldB === testStructSome.fieldB);
	assert(deStructSome.fieldC.length === 3);
	assert(deStructSome.fieldC[0].amount.toNumber() === otherStateOne.amount.toNumber());
	assert(deStructSome.fieldC[0].timestamp.toNumber() === otherStateOne.timestamp.toNumber());
	assert(deStructSome.fieldC[1].amount.toNumber() === otherStateTwo.amount.toNumber());
	assert(deStructSome.fieldC[1].timestamp.toNumber() === otherStateTwo.timestamp.toNumber());
	assert(deStructSome.fieldC[2].amount.toNumber() === otherStateThree.amount.toNumber());
	assert(deStructSome.fieldC[2].timestamp.toNumber() === otherStateThree.timestamp.toNumber());

	const serStructSome = Array.from(serialize(SCHEMA, testStructSome));
	assert(serStructSome.length === structData.testStructSome.length);
	for (var i = 0; i < serStructSome.length; i++) {
			assert(serStructSome[i] === structData.testStructSome[i]);
	}
}


export function testStruct2() {
	// test struct with none field
	const deStructNone = deserializeUnchecked(SCHEMA, TestStruct, Buffer.from(structData.testStructNone));
	assert(deStructNone.fieldA.toNumber() === testStructNone.fieldA.toNumber());
	assert(deStructNone.fieldB === testStructNone.fieldB);
	assert(deStructNone.fieldC == null);

	const serStructNone = Array.from(serialize(SCHEMA, testStructNone));
	assert(serStructNone.length === structData.testStructNone.length);
	for (var i = 0; i < serStructNone.length; i++) {
			assert(serStructNone[i] === structData.testStructNone[i]);
	}

	const deTupleStruct = deserializeUnchecked(SCHEMA, TupleStruct, Buffer.from(structData.tupleStruct));
	assert(deTupleStruct.unnamed_0 === tupleStruct.unnamed_0);
	assert((deTupleStruct.unnamed_1 << 32) >> 32 === tupleStruct.unnamed_1);
	assert(deTupleStruct.unnamed_2.amount.toNumber() === tupleStruct.unnamed_2.amount.toNumber());
	assert(deTupleStruct.unnamed_2.timestamp.toNumber() === tupleStruct.unnamed_2.timestamp.toNumber());
}
