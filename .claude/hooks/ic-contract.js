#!/usr/bin/env node
"use strict";

// The compact machine contract for the one inquiry spine.  It is an
// implementation projection of the construction specification and Questions.txt,
// never an independent semantic or procedural authority.

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const CONTRACT_PATH = "formal-successor/INQUIRY_SPINE_CONTRACT.json";
const CORPUS_PATH = "formal-successor/Questions.txt";

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function read(root) {
  const corpusBytes = fs.readFileSync(path.join(root, ...CORPUS_PATH.split("/")));
  const contractBytes = fs.readFileSync(path.join(root, ...CONTRACT_PATH.split("/")));
  const contract = JSON.parse(contractBytes.toString("utf8"));
  if (contract.schema !== 5) throw new Error("inquiry spine contract must use schema 5");
  if (contract.corpus?.path !== CORPUS_PATH || contract.corpus.sha256 !== sha256(corpusBytes)) {
    throw new Error("inquiry spine contract is detached from Questions.txt");
  }
  const expected = ["RELATE", "OPEN", "TURN", "RETURN", "DISTINGUISH", "FOLD", "CARRY"];
  if (JSON.stringify(contract.model_recurrence) !== JSON.stringify(expected)) {
    throw new Error("contract must expose exactly the one inquiry recurrence");
  }
  if (!Array.isArray(contract.question_forms) || !Array.isArray(contract.generator_registry) ||
      !Array.isArray(contract.path_generators) || !Array.isArray(contract.method_contracts)) {
    throw new Error("inquiry spine contract is incomplete");
  }
  const forms = new Set(contract.question_forms.map((form) => form.id));
  if (forms.size !== contract.question_forms.length) throw new Error("duplicate question form");
  const covered = new Set();
  for (const generator of contract.generator_registry) {
    for (const form of generator.question_forms) {
      if (!forms.has(form)) throw new Error(`${generator.id}: undeclared question form ${form}`);
      covered.add(form);
    }
  }
  for (const form of forms) if (!covered.has(form)) throw new Error(`unreachable question form ${form}`);
  return {
    contract,
    contractBytes,
    contractDigest: sha256(contractBytes),
    corpusBytes,
    corpusDigest: sha256(corpusBytes),
  };
}

function formMap(contract) {
  return new Map(contract.question_forms.map((form) => [form.id, form]));
}

module.exports = { CONTRACT_PATH, CORPUS_PATH, formMap, read, sha256 };
