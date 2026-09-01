#!/usr/bin/env node
"use strict";

const A=require("node:assert/strict"),C=require("node:child_process"),H=require("node:crypto"),F=require("node:fs"),O=require("node:os"),P=require("node:path");
const R=P.resolve(__dirname,".."),M="InquiryCalculus.Legacy.V20.PatternSeparation",L="formal/InquiryCalculus/Legacy/V20/PatternSeparation.lean",D="1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89";
const S=new Map([["PRED-TEX-PROSE-2AE6CB059F5D7C1B",[4812,4812,"Ambiguous"]],["PRED-TEX-DISPLAY-AB8FA67E22055228",[4813,4817,"Ambiguous"]],["PRED-TEX-PROSE-F97BF25398F68394",[4818,4818,"Unproved"]],["PRED-TEX-PROSE-4D7ED37A107B93CB",[4820,4820,"Ambiguous"]]]),h=x=>H.createHash("sha256").update(x).digest("hex");
function main(){
  A.ok(process.argv.slice(2).every(x=>x==='--compile'));
  const r=x=>F.readFileSync(P.join(R,x)),t=r('Inquiry_Calculus_v2_0.tex'),c=JSON.parse(r('formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json')),l=r(L).toString(),d=r('formal-successor/PHASE_B_PATTERN_SEPARATION.md').toString(),z=t.toString().replace(/\r\n?/gu,'\n').split('\n');
  A.equal(h(t),D);
  for(const[id,[a,b,s]]of S){const q=c.records.filter(x=>x.source_id===id);A.equal(q.length,1,id);const x=q[0],e=z.slice(a-1,b).map(x=>x.trimEnd()).join('\n').trim();A.equal(x.disposition,'LegacyObligation',id);A.equal(x.legacy_obligation.status,s,id);A.deepEqual([x.source.start_line,x.source.end_line],[a,b],id);A.equal(x.source.revision,`sha256:${D}`,id);A.equal(x.source.sha256,h(e),id);A.equal(x.source_excerpt_sha256,h(e),id);A.match(x.destination,new RegExp(`/Obligations/pattern-separation-and-completion/${id}$`,'u'))}
  for(const x of['PatternBoundary','represent','similar','consequence','protectedDifferent','RequiresSeparation','CompletionBoundary','live','protectedEquivalent','approximatePermitsAmbiguity','CompletionLicensed','mergedDoesNotSeparate','splitSeparates','exactCompletionIsLicensed','inequivalentCompletionWithoutLicenceFails','approximateLicencePermitsAmbiguity'])A.match(l,new RegExp(`\\b${x}\\b`,'u'));
  A.match(d,/four exact `LegacyObligation` records at v2\.0 lines 4812–4820/u);A.doesNotMatch(l,/\b(?:sorry|admit|axiom|unsafe|native_decide|Classical\.choose)\b/u);A.match(r('formal/InquiryCalculus.lean').toString(),/^import InquiryCalculus\.Legacy\.V20\.PatternSeparation\r?$/mu);
  console.log(`PASS exact pattern-separation sources and merger/completion contrasts; module sha256 ${h(l)}`);if(!process.argv.includes('--compile'))return;
  const dir=F.mkdtempSync(P.join(O.tmpdir(),'ic-pattern-')),run=a=>C.spawnSync('lake',a,{cwd:P.join(R,'formal'),encoding:'utf8',windowsHide:true}),p=(n,b,re=false,own=false)=>{const f=P.join(dir,`${n}.lean`);F.writeFileSync(f,(own?'':`import ${M}\n`)+b);const z=run(['env','lean',f]),o=z.stdout+z.stderr;if(re){A.notEqual(z.status,0,`accepted ${n}`);A.match(o,/error(?:\([^)]*\))?:/u)}else A.equal(z.status,0,o);return o};
  const b=run(['build',M,'--wfail']);A.equal(b.status,0,b.stdout+b.stderr);
  const a=['Countermodel.mergedDoesNotSeparate','Countermodel.splitSeparates','Countermodel.exactCompletionIsLicensed','Countermodel.inequivalentCompletionWithoutLicenceFails','Countermodel.approximateLicencePermitsAmbiguity'],o=p('contracts',a.map(x=>`#print axioms ${M}.${x}`).join('\n'));
  for(const x of a)A.match(o,new RegExp(`'${M.replaceAll('.','\\.')}.${x.replaceAll('.','\\.')}' does not depend on any axioms`));
  for(const[n,before,after]of[['similarity','  similar : Event → Event → Prop','  similar : True'],['difference','  protectedDifferent : Consequence → Consequence → Prop','  protectedDifferent : True'],['separation','def RequiresSeparation','def RequiresSeparationRemoved'],['completion-equivalence','  protectedEquivalent : Completion → Completion → Prop','  protectedEquivalent : True'],['approximate','  approximatePermitsAmbiguity : Prop','  approximatePermitsAmbiguity : True'],['merged-foil','def merged','def mergedRemoved']]){const x=l.replace(before,after);A.notEqual(x,l,n);p(`drop-${n}`,x,true,true)}
  console.log(`PASS six pattern-separation ablations and ${a.length} axiom-free proof audits`);
}
main();
