#!/usr/bin/env node
"use strict";

// Bounded first-order engineering question derivation. Inputs must explicitly
// represent incidence, reverse use, discriminator domain, or exchangeability.
// Generated occurrences are Unknown/non-executable and are never selected here.
const crypto = require("node:crypto");
const canonical = (value) => Array.isArray(value) ? `[${value.map(canonical).join(",")}]` :
  value && typeof value === "object" ? `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}` : JSON.stringify(value);
const digest = (value) => crypto.createHash("sha256").update(canonical(value)).digest("hex");
const nonempty = (value, label) => { if (typeof value !== "string" || !value.trim()) throw new Error(`${label} must be nonempty`); };

function validateSurface(surface) {
  if (!surface || typeof surface !== "object" || !Array.isArray(surface.relations) ||
      !Array.isArray(surface.questions) || !Array.isArray(surface.discriminators)) throw new Error("surface requires relation, question and discriminator arrays");
  const relationIds = new Set();
  for (const relation of surface.relations) {
    for (const key of ["id", "source", "target", "path"]) nonempty(relation[key], `relation.${key}`);
    if (!Array.isArray(relation.dependencies)) throw new Error("relation dependencies must be an array");
    if (relationIds.has(relation.id)) throw new Error(`duplicate relation ${relation.id}`);
    relationIds.add(relation.id);
  }
  for (const relation of surface.relations) if (relation.reverse_id !== undefined && !relationIds.has(relation.reverse_id)) throw new Error(`undeclared reverse relation ${relation.reverse_id}`);
  for (const question of surface.questions) {
    for (const key of ["occurrence", "path", "context"]) nonempty(question[key], `question.${key}`);
    if (typeof question.exchangeable !== "boolean" || !Array.isArray(question.dependencies)) throw new Error("question exchangeability/dependencies malformed");
  }
  for (const discriminator of surface.discriminators) {
    for (const key of ["id", "domain", "path"]) nonempty(discriminator[key], `discriminator.${key}`);
    if (!Array.isArray(discriminator.dependencies)) throw new Error("discriminator dependencies malformed");
  }
}

function generate(surface, contract) {
  validateSurface(surface);
  if (!Array.isArray(contract.path_generators)) throw new Error("missing path generator contract");
  const forms = new Map(contract.question_forms.map((form) => [form.id, form]));
  const families = new Map(contract.path_generators.map((family) => [family.id, family]));
  const output = [];
  function emit(familyId, inputs, context, path, dependencies, detail) {
    const family = families.get(familyId); if (!family) throw new Error(`undeclared generator family ${familyId}`);
    const form = forms.get(family.question_form); if (!form) throw new Error(`undeclared generator form ${family.question_form}`);
    const derivation = { schema: 1, family: familyId, inputs, detail };
    const identity = digest([family.question_form, derivation, context, path, dependencies]);
    const generatorIds = contract.generator_registry.filter((entry) => entry.question_forms.includes(family.question_form)).map((entry) => entry.id);
    output.push({ occurrence: `QG-${identity}`, rendering: `RG-${identity}`, question_form: family.question_form,
      prompt: `${form.prompt}\nDerived from represented ${family.requires.join(" + ")}: ${detail}.`, source_lines: form.source_lines,
      generator_ids: generatorIds, context, path, dependencies: [...new Set(dependencies)], derivation,
      disposition: "Unknown", executable: false });
  }
  for (const relation of surface.relations) emit("DIRECT", [relation.id], `relation:${relation.id}`, `${relation.path}/direct`, [relation.id, ...relation.dependencies], `relation ${relation.id}`);
  for (const relation of surface.relations) if (relation.reverse_id) emit("RECIPROCAL", [relation.id, relation.reverse_id], `reciprocal:${relation.id}:${relation.reverse_id}`, `${relation.path}/reciprocal`, [relation.id, relation.reverse_id, ...relation.dependencies], `declared opposed uses ${relation.id} and ${relation.reverse_id}`);
  for (const left of surface.relations) for (const right of surface.relations) if (left.id !== right.id && left.target === right.source) {
    emit("COMPOSE", [left.id, right.id], `composition:${right.id}∘${left.id}`, `${left.path}/then/${right.path}`, [left.id, right.id, ...left.dependencies, ...right.dependencies], `typed incidence ${left.source} -> ${left.target} -> ${right.target}`);
  }
  for (const relation of surface.relations) for (const discriminator of surface.discriminators) if (relation.target === discriminator.domain) {
    emit("TRANSPORT", [relation.id, discriminator.id], `transport:${discriminator.id}∘${relation.id}`, `${relation.path}/transport/${discriminator.path}`, [relation.id, discriminator.id, ...relation.dependencies, ...discriminator.dependencies], `downstream discriminator ${discriminator.id} transported through ${relation.id}`);
  }
  for (const question of surface.questions) emit("QUESTION_SUBJECT", [question.occurrence], `question-subject:${question.context}`, `${question.path}/question-subject`, [question.occurrence, ...question.dependencies], `ordinary question occurrence ${question.occurrence}`);
  for (let i = 0; i < surface.questions.length; i += 1) for (let j = i + 1; j < surface.questions.length; j += 1) {
    const left = surface.questions[i], right = surface.questions[j];
    if (left.context === right.context && left.exchangeable && right.exchangeable) emit("PERMUTE", [left.occurrence, right.occurrence], `permutation:${left.context}`, `${left.context}/permute`, [left.occurrence, right.occurrence, ...left.dependencies, ...right.dependencies], `declared exchangeable order ${left.occurrence};${right.occurrence}`);
  }
  for (const relation of surface.relations) emit("REGENERATE", [relation.id], `regeneration:${relation.id}`, `${relation.path}/regenerate`, [relation.id, ...relation.dependencies], `question basis that could regenerate ${relation.id}`);
  return output;
}

function materialize(product, context, contract) {
  if (!product?.inquiry_generator_surface) throw new Error("missing inquiry_generator_surface");
  const surface = product.inquiry_generator_surface;
  validateSurface(surface);
  const represented = [
    ...surface.relations.map((relation) => relation.id),
    ...surface.questions.map((question) => question.occurrence),
    ...surface.discriminators.map((discriminator) => discriminator.id),
  ];
  const missingDependencies = represented.filter((id) => !product.dependencies.includes(id));
  if (missingDependencies.length) throw new Error(`generator surface omits represented dependencies: ${missingDependencies.join(",")}`);
  for (const relation of surface.relations) {
    if (!context.products.has(relation.id)) throw new Error(`generator surface relation is not reified: ${relation.id}`);
  }
  for (const question of surface.questions) {
    if (!context.questions.has(question.occurrence)) throw new Error(`generator surface question is not represented: ${question.occurrence}`);
  }
  for (const discriminator of surface.discriminators) {
    if (!context.products.has(discriminator.id)) throw new Error(`generator surface discriminator is not reified: ${discriminator.id}`);
  }
  return generate(surface, contract).map((member) => {
    const derivation = { ...member.derivation, surface_product: product.id };
    const dependencies = [...new Set([product.id, ...member.dependencies, ...product.dependencies])];
    const identity = digest([member.question_form, derivation, member.context, member.path, dependencies]);
    return { ...member, occurrence: `QG-${identity}`, rendering: `RG-${identity}`, dependencies, derivation };
  });
}

function validateProduct(product, context, contract) {
  if (!Object.hasOwn(product, "inquiry_generator_surface")) return;
  materialize(product, context, contract);
}

function validateMember(member, context, contract) {
  const surfaceId = member.derivation?.surface_product;
  if (!surfaceId) throw new Error("derived question lacks generator surface ancestry");
  const product = context.products.get(surfaceId);
  if (!product?.inquiry_generator_surface) throw new Error(`missing generator surface product ${surfaceId}`);
  const expected = materialize(product, context, contract).find((candidate) => candidate.occurrence === member.occurrence);
  const immutable = (question) => {
    const { disposition: _disposition, executable: _executable, ...identity } = question;
    return identity;
  };
  if (!expected || canonical(immutable(expected)) !== canonical(immutable(member))) {
    throw new Error("derived question differs from its reified generator surface");
  }
  if (member.executable && member.dependencies.some((id) => context.invalidated.has(id))) {
    throw new Error("derived question depends on invalidated material and cannot execute");
  }
}

function validateRendering(member, contract) {
  const family = contract.path_generators?.find((candidate) => candidate.id === member.derivation?.family);
  const form = contract.question_forms.find((candidate) => candidate.id === member.question_form);
  if (!family || !form || family.question_form !== member.question_form || member.derivation?.schema !== 1 ||
      !Array.isArray(member.derivation.inputs) || typeof member.derivation.detail !== "string" || !member.derivation.detail) {
    throw new Error("derived question has no declared family/form rendering");
  }
  const prompt = `${form.prompt}\nDerived from represented ${family.requires.join(" + ")}: ${member.derivation.detail}.`;
  const identity = digest([member.question_form, member.derivation, member.context, member.path, member.dependencies]);
  if (member.prompt !== prompt || member.rendering !== `RG-${identity}` || member.occurrence !== `QG-${identity}`) {
    throw new Error("derived question rendering identity or prompt changed");
  }
}

module.exports = { canonical, digest, validateSurface, generate, materialize, validateProduct, validateMember, validateRendering };
