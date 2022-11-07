import {
    BTreeWrapper,
    SCHEMA
} from "./schema";
import { PublicKey } from "@solana/web3.js";

import { serialize, deserializeUnchecked } from "borsh";
import assert from "assert";
import fs from "fs";

// Read serialized data from rust
var data = fs.readFileSync("../test-data/test_btree.json");
const btreeData = JSON.parse(data.toString());

// MAP TESTS
let pubkey = [
    10, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0
];
const id0 = [
    100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0
];

let map0 = new Map();
const pubkey0 = new PublicKey(pubkey);
map0.set(id0, pubkey0);
const id1 = [
    100, 101, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0
];
pubkey[1] = 11;
const pubkey1 = new PublicKey(pubkey);
map0.set(id1, pubkey1);
const id2 = [
    100, 101, 102, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0
];
pubkey[2] = 22;
const pubkey2 = new PublicKey(pubkey);
map0.set(id2, pubkey2);
const id3 = [
    100, 101, 102, 103, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0
];
pubkey[3] = 33;
const pubkey3 = new PublicKey(pubkey);
map0.set(id3, pubkey3);
const id4 = [
    100, 101, 102, 103, 104, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0
];
pubkey[4] = 44;
const pubkey4 = new PublicKey(pubkey);
map0.set(id4, pubkey4);

let map1 = new Map();
map1.set("hello", 23);
map1.set("bello", 33);
map1.set("yello", null);
map1.set("zello", 44);

let map2 = new Map();
map2.set(168, "value");
map2.set(169, "values");

export function testMap() {
	const wrapper = new BTreeWrapper({ map0, map1, map2 });
	const serialized = serialize(SCHEMA, wrapper);
	assert(serialized.length === btreeData.length);
	const wrapperDeserialized = deserializeUnchecked(SCHEMA, BTreeWrapper, Buffer.from(btreeData));

	let entries = wrapperDeserialized.map0.entries();
	var [key, value] = entries.next().value;
	assert(key.toString() === id0.toString());
	assert(value.toString() === pubkey0.toString());
	var [key, value] = entries.next().value;
	assert(key.toString() === id1.toString());
	assert(value.toString() === pubkey1.toString());
	var [key, value] = entries.next().value;
	assert(key.toString() === id2.toString());
	assert(value.toString() === pubkey2.toString());
	var [key, value] = entries.next().value;
	assert(key.toString() === id3.toString());
	assert(value.toString() === pubkey3.toString());
	var [key, value] = entries.next().value;
	assert(key.toString() === id4.toString());
	assert(value.toString() === pubkey4.toString());

	assert(wrapperDeserialized.map1.get("hello") === 23);
	assert(wrapperDeserialized.map1.get("bello") === 33);
	assert(wrapperDeserialized.map1.get("yello") == null);
	assert(wrapperDeserialized.map1.get("zello") === 44);
	assert(wrapperDeserialized.map2.get(168) === "value");
	assert(wrapperDeserialized.map2.get(169) === "values");
}
