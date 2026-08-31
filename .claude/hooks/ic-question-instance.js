#!/usr/bin/env node
"use strict";

// A bounded, provisional engineering constructor. Carrier names and relation
// signatures are supplied declarations, NOT inferred successor types/theorems.
// A seed opens one joint tuple. It does not assert existence, invert a relation,
// discharge a question, choose a question, or create an event.
const crypto = require("node:crypto");

function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value && typeof value === "object") return `{${Object.keys(value).sort()
    .map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}`;
  return JSON.stringify(value);
}
const digest = (value) => crypto.createHash("sha256").update(canonical(value)).digest("hex");
function object(value, keys, label) {
  if (!value || Array.isArray(value) || typeof value !== "object" ||
      Object.keys(value).some((key) => !keys.includes(key)) || keys.some((key) => !Object.hasOwn(value, key))) {
    throw new Error(`${label}: exact fields required: ${keys.join(",")}`);
  }
}
function string(value, label) {
  if (typeof value !== "string" || !value.trim()) throw new Error(`${label}: nonempty string required`);
}
function signature(value) {
  object(value, ["label", "roles"], "inquiry_relation");
  string(value.label, "relation label");
  if (!Array.isArray(value.roles) || !value.roles.length) throw new Error("relation requires ordered roles");
  const names = new Set();
  for (const role of value.roles) {
    object(role, ["name", "carrier"], "role");
    string(role.name, "role name"); string(role.carrier, "role carrier");
    if (names.has(role.name)) throw new Error(`duplicate role ${role.name}`);
    names.add(role.name);
  }
}

function compileSeed(product, context) {
  const seed = product.inquiry_seed;
  object(seed, ["question_form", "relation_product", "bindings", "open_roles", "path"], "inquiry_seed");
  for (const key of ["question_form", "relation_product", "path"]) string(seed[key], key);
  const relation = context.products.get(seed.relation_product);
  if (!relation?.inquiry_relation) throw new Error(`missing relation product ${seed.relation_product}`);
  signature(relation.inquiry_relation);
  if (!seed.bindings || Array.isArray(seed.bindings) || typeof seed.bindings !== "object") throw new Error("bindings must be a role map");
  if (!Array.isArray(seed.open_roles) || !seed.open_roles.length ||
      new Set(seed.open_roles).size !== seed.open_roles.length) throw new Error("open_roles must be a nonempty distinct tuple");
  const roles = relation.inquiry_relation.roles;
  const names = new Set(roles.map((role) => role.name));
  if ([...Object.keys(seed.bindings), ...seed.open_roles].some((name) => !names.has(name))) throw new Error("undeclared role in seed");
  // Role order is significant; do not silently canonicalize an answer tuple.
  if (canonical(seed.open_roles) !== canonical(roles.filter((role) => seed.open_roles.includes(role.name)).map((role) => role.name))) {
    throw new Error("open tuple must preserve declared role order");
  }
  const dependencies = [product.id, relation.id];
  for (const role of roles) {
    const bound = Object.hasOwn(seed.bindings, role.name);
    const open = seed.open_roles.includes(role.name);
    if (bound === open) throw new Error(`role ${role.name} must be exactly bound or open (no capture)`);
    if (!bound) continue;
    const ref = seed.bindings[role.name];
    object(ref, ["kind", "id"], "binding reference"); string(ref.id, "reference id");
    let carrier;
    if (ref.kind === "product") {
      const target = context.products.get(ref.id);
      if (!target) throw new Error(`dangling product reference ${ref.id}`);
      carrier = target.inquiry_carrier;
      dependencies.push(ref.id);
    } else if (ref.kind === "question") {
      const target = context.questions.get(ref.id);
      if (!target) throw new Error(`dangling question reference ${ref.id}`);
      carrier = "Question";
      dependencies.push(...target.dependencies);
    } else throw new Error(`unknown reference kind ${ref.kind}`);
    if (carrier !== role.carrier) throw new Error(`carrier mismatch at role ${role.name}: ${carrier} != ${role.carrier}`);
  }
  for (const key of ["horizon", "coverage", "applicability"]) string(product[key], key);
  if (!Array.isArray(product.dependencies) || product.dependencies.some((id) => typeof id !== "string" || !id.trim())) {
    throw new Error("seed dependencies must name product identities");
  }
  const required = [...new Set(dependencies)];
  // The seed's declared ancestry must include every product read by generation.
  // Extra provenance dependencies are retained, not silently discarded.
  if (required.filter((id) => id !== product.id).some((id) => !product.dependencies.includes(id))) {
    throw new Error("seed dependencies omit a relation, filling, or question dependency");
  }
  return {
    instance: {
      schema: 1, seed_product: product.id, relation_product: relation.id,
      relation: relation.inquiry_relation, bindings: seed.bindings, open_roles: seed.open_roles,
      horizon: product.horizon, coverage: product.coverage, applicability: product.applicability,
    },
    dependencies: [...new Set([...required, ...product.dependencies])],
    question_form: seed.question_form, path: seed.path,
  };
}

function validateProduct(product, context) {
  if (Object.hasOwn(product, "inquiry_carrier")) string(product.inquiry_carrier, "inquiry_carrier");
  if (Object.hasOwn(product, "inquiry_relation")) signature(product.inquiry_relation);
  if (Object.hasOwn(product, "inquiry_seed")) compileSeed(product, context);
}

function render(formPrompt, instance) {
  const tuple = instance.relation.roles.filter((role) => instance.open_roles.includes(role.name))
    .map((role) => `${JSON.stringify(role.name)}:${JSON.stringify(role.carrier)}`).join(", ");
  const roles = instance.relation.roles.map((role) => {
    const ref = Object.hasOwn(instance.bindings, role.name) ? instance.bindings[role.name] : null;
    return `${JSON.stringify(role.name)}:${JSON.stringify(role.carrier)}=${ref ? `${ref.kind}:${JSON.stringify(ref.id)}` : "?"}`;
  }).join(", ");
  return `${formPrompt}\nOpen jointly (${tuple}) in ${JSON.stringify(instance.relation.label)} (${roles}). ` +
    `Unfilled roles are unknown, not asserted witnesses. Applicability: ${JSON.stringify(instance.applicability)}. ` +
    `Horizon: ${JSON.stringify(instance.horizon)}. Coverage: ${JSON.stringify(instance.coverage)}.`;
}

function materialize(product, context, contract) {
  const compiled = compileSeed(product, context);
  const form = contract.question_forms.find((entry) => entry.id === compiled.question_form);
  if (!form) throw new Error(`undeclared question form ${compiled.question_form}`);
  const instance = compiled.instance;
  const member = {
    question_form: compiled.question_form,
    prompt: render(form.prompt, instance), source_lines: form.source_lines,
    generator_ids: contract.generator_registry.filter((entry) => entry.question_forms.includes(form.id)).map((entry) => entry.id),
    context: `relation:${instance.relation_product}`, path: compiled.path,
    dependencies: compiled.dependencies, relational_instance: instance,
    // Formability supplies no execution capability or productive/required warrant.
    disposition: "Unknown", executable: false,
  };
  const identity = renderingIdentity(member);
  return { occurrence: `QI-${digest([identity, compiled.path])}`, rendering: `RI-${identity}`, ...member };
}

function renderingIdentity(member) {
  return digest([member.question_form, member.relational_instance, member.prompt, member.source_lines,
    member.generator_ids, member.context, member.path]);
}

function validateMember(member, context) {
  const instance = member.relational_instance;
  if (!instance || instance.schema !== 1) throw new Error("unsupported relational_instance schema");
  const product = context.products.get(instance.seed_product);
  if (!product?.inquiry_seed) throw new Error("missing reified inquiry seed");
  const compiled = compileSeed(product, context);
  if (canonical(instance) !== canonical(compiled.instance)) throw new Error("instance meaning differs from reified seed");
  if (member.question_form !== compiled.question_form || member.path !== compiled.path ||
      canonical(member.dependencies) !== canonical(compiled.dependencies)) throw new Error("instance path, form, or dependencies changed");
  const identity = renderingIdentity(member);
  if (member.rendering !== `RI-${identity}` || member.occurrence !== `QI-${digest([identity, compiled.path])}`) {
    throw new Error("instance rendering/occurrence identity changed");
  }
  if (member.executable && compiled.dependencies.some((id) => context.invalidated.has(id))) {
    throw new Error("instance depends on invalidated material and cannot execute");
  }
}

module.exports = { canonical, digest, compileSeed, validateProduct, render, renderingIdentity, materialize, validateMember };
