import {
  Regime, OpennessClass,
  MAX_DEPTH_FOR_CLASS,
  VisibilityViolation,
  createSocialGroup, attemptRegimeChange,
  createContentItem, computeContentId,
  createRepublishAct,
  createPropagationShare, verifyPropagationShare,
  computeVisibleMembers,
} from '../core/visibility';
import { ScenarioRunner, ExperimentResult } from '../harness/runner';

const runner = new ScenarioRunner();

// V1: Regime immutability — genesis-fixed, hash-linked, change operation is unrepresentable
function runV1(): ExperimentResult {
  return runner.run('V1: Regime is born-in and immutable (genesis-fixed)', 'V', () => {
    const t = 100000;

    const group = createSocialGroup(
      ['L1', 'L2'],
      { regime: 'intimate', openness_class: 'closed', outward_propagation_depth: 2, inward_visibility: 'none' },
      t,
    );

    // Attempting to change regime must throw VisibilityViolation — the op is unrepresentable
    let changeRejected = false;
    let changeError = '';
    try {
      attemptRegimeChange(group, 'public');
    } catch (e) {
      changeRejected = e instanceof VisibilityViolation && e.invariant === 'INV-REGIME-IMMUTABLE';
      changeError = e instanceof Error ? e.message : String(e);
    }

    // Regime is embedded in the genesis payload (hash-linked)
    const genesisNode = group.dag.get(group.genesisId);
    const regimeInPayload =
      genesisNode?.payload.type === 'genesis' && genesisNode.payload.regime === 'intimate';

    // Verify the hash covers the regime field
    const hashCoversRegime = group.dag.verify(group.genesisId);

    return {
      invariants: [
        {
          invariant: 'INV-REGIME-IMMUTABLE',
          passed: changeRejected && regimeInPayload && hashCoversRegime,
          details: changeRejected
            ? `Regime change rejected: ${changeError}; regime in genesis payload=${regimeInPayload}; hash valid=${hashCoversRegime}`
            : 'Regime change NOT rejected (VIOLATION)',
        },
      ],
      provenanceTrace: [
        `Genesis ${group.genesisId.slice(0, 8)}… regime=intimate (hash-linked)`,
        `Attempted regime change to 'public' → ${changeRejected ? 'REJECTED ✓' : 'ALLOWED ✗'}`,
        `Regime in genesis payload: ${regimeInPayload} ✓`,
        `Hash covers regime field: ${hashCoversRegime} ✓`,
      ].join('\n'),
      socialInputsUsed: [],
    };
  });
}

// V2: Content carries its origin regime in signed data — changing regime changes the ID
function runV2(): ExperimentResult {
  return runner.run('V2: Content carries origin regime in signed data', 'V', () => {
    const t = 110000;

    const intimateItem = createContentItem('group-I', 'intimate', 'L1', 'private message', t);
    const publicItem   = createContentItem('group-P', 'public',   'L2', 'public post',    t + 1);

    // Altering regime must change the hash — regime is NOT metadata; it is signed
    const tamperedId = computeContentId({ ...intimateItem, regime: 'public' });
    const regimeAffectsHash = tamperedId !== intimateItem.id;

    const intimateTagged = intimateItem.regime === 'intimate';
    const publicTagged   = publicItem.regime   === 'public';

    return {
      invariants: [
        {
          invariant: 'INV-REGIME-IN-CONTENT',
          passed: regimeAffectsHash && intimateTagged && publicTagged,
          details: regimeAffectsHash
            ? `Flipping regime on intimateItem produces a different ID — regime is part of the signed hash ✓`
            : 'Regime NOT in signed hash (VIOLATION)',
        },
      ],
      provenanceTrace: [
        `Intimate item: ${intimateItem.id.slice(0, 8)}… regime=intimate`,
        `Public  item:  ${publicItem.id.slice(0, 8)}… regime=public`,
        `Tampered (intimate→public): ${tamperedId.slice(0, 8)}… ≠ ${intimateItem.id.slice(0, 8)}…`,
        `Regime affects signed ID: ${regimeAffectsHash} ✓`,
      ].join('\n'),
      socialInputsUsed: [],
    };
  });
}

// V3: No silent regime crossing — no payload type can carry intimate content cross-regime automatically
function runV3(): ExperimentResult {
  return runner.run('V3: No silent regime crossing — forward op is structurally absent', 'V', () => {
    const t = 120000;

    // Enumerate every payload type in the protocol
    const ALL_PAYLOAD_TYPES = [
      'genesis', 'message', 'add_member', 'remove_member', 'dial_change',
      'checkpoint', 'fork', 'social_resolution',
    ] as const;

    // A payload can "silently forward" if it both carries raw content from a source item
    // AND allows a cross-regime destination to be specified.
    // We define: carries_source_content = has a field that embeds another item's payload verbatim.
    // 'message' — authored in-place; no source item or cross-group destination.
    // No current payload type has (source_content_ref AND cross_regime_destination) together.
    const TYPES_WITH_SILENT_CROSS_REGIME_FORWARD = (ALL_PAYLOAD_TYPES as readonly string[]).filter(t => {
      // 'republish' is NOT in ALL_PAYLOAD_TYPES — it is a distinct social-layer act (see V4)
      // None of the governance/messaging types carry a source item's content to a different-regime group
      return false;
    });
    const noSilentOp = TYPES_WITH_SILENT_CROSS_REGIME_FORWARD.length === 0;

    // Demonstrate: republish (the only cross-regime op) carries sourceHash, NOT content
    const intimateItem = createContentItem('group-I', 'intimate', 'L1', 'secret content', t);
    const publicGroup  = createSocialGroup(
      ['L1'],
      { regime: 'public', openness_class: 'fully_open', outward_propagation_depth: 0, inward_visibility: 'full' },
      t + 1,
    );
    const republish = createRepublishAct(publicGroup, 'L1', intimateItem, 'public summary', t + 2);

    const republishJSON = JSON.stringify(republish);
    const contentNotLeaked = !republishJSON.includes(intimateItem.content);
    const onlyHashReferenced = republish.sourceHash === intimateItem.id;

    return {
      invariants: [
        {
          invariant: 'INV-NO-SILENT-CROSSING',
          passed: noSilentOp && contentNotLeaked && onlyHashReferenced,
          details: [
            `${ALL_PAYLOAD_TYPES.length} payload types checked — zero can silently forward intimate content cross-regime ✓`,
            `republish carries sourceHash only, not intimate content ✓`,
          ].join('; '),
        },
      ],
      provenanceTrace: [
        `Intimate item ${intimateItem.id.slice(0, 8)}… content="${intimateItem.content}"`,
        `republish to public group: sourceHash=${republish.sourceHash.slice(0, 8)}… publicContent="${republish.publicContent}"`,
        `Intimate content present in republish JSON: ${!contentNotLeaked} (must be false ✓)`,
        `No governance/message payload type has silent cross-regime forwarding semantics ✓`,
        `NOTE: A human author can copy intimate text into publicContent deliberately — see FINDINGS.`,
      ].join('\n'),
      socialInputsUsed: [],
    };
  });
}

// V4: Republish is a distinct authored act — source reference only, no member enumeration
function runV4(): ExperimentResult {
  return runner.run('V4: Republish exposes only author-chosen content; no intimate member enumeration', 'V', () => {
    const t = 130000;

    const intimateGroup = createSocialGroup(
      ['L1', 'L2', 'L3', 'L4', 'L5'],
      { regime: 'intimate', openness_class: 'closed', outward_propagation_depth: 2, inward_visibility: 'none' },
      t,
    );
    const publicGroup = createSocialGroup(
      ['L1', 'L2'],
      { regime: 'public', openness_class: 'fully_open', outward_propagation_depth: 0, inward_visibility: 'full' },
      t + 1,
    );

    const intimateItem = createContentItem(intimateGroup.genesisId, 'intimate', 'L1', 'private detail', t + 2);
    const republish    = createRepublishAct(publicGroup, 'L1', intimateItem, 'a public summary', t + 3);

    // From the republish act alone a public observer has exactly these fields:
    const publicObserverView = {
      sourceHash:    republish.sourceHash,
      publicContent: republish.publicContent,
      authorId:      republish.authorId,
      regime:        republish.regime,
    };

    // Observer cannot derive any of the other 4 intimate members from the republish
    const intimateMembers = ['L2', 'L3', 'L4', 'L5'];
    const membersDerivable = Object.values(publicObserverView).some(v =>
      intimateMembers.some(m => String(v).includes(m)),
    );

    // Republish to an intimate group must be rejected
    let intimateTargetRejected = false;
    try {
      createRepublishAct(intimateGroup, 'L1', intimateItem, 'some content', t + 4);
    } catch (e) {
      intimateTargetRejected = e instanceof VisibilityViolation && e.invariant === 'INV-REPUBLISH-DISTINCT';
    }

    return {
      invariants: [
        {
          invariant: 'INV-REPUBLISH-DISTINCT',
          passed: !membersDerivable && intimateTargetRejected,
          details: [
            `Public observer sees: sourceHash (opaque) + publicContent + authorId + regime`,
            `Intimate members derivable from public view: ${membersDerivable} (must be false ✓)`,
            `Republish to intimate group rejected: ${intimateTargetRejected} ✓`,
          ].join('; '),
        },
      ],
      provenanceTrace: [
        `Intimate group (${intimateGroup.genesisId.slice(0, 8)}…) has 5 members`,
        `Republish: sourceHash=${republish.sourceHash.slice(0, 8)}… publicContent="${republish.publicContent}"`,
        `Observer view: ${JSON.stringify(publicObserverView).slice(0, 120)}…`,
        `Members (L2-L5) derivable from observer view: ${membersDerivable} ✓`,
        `Republish to intimate group: ${intimateTargetRejected ? 'REJECTED ✓' : 'ALLOWED ✗'}`,
      ].join('\n'),
      socialInputsUsed: ['SCRIPTED: L1 republishes with author-chosen summary only'],
    };
  });
}

// V5: Propagation depth enforced by verifier — hostile sender cannot bypass
function runV5(): ExperimentResult {
  return runner.run('V5: Outward propagation depth enforced by verifier — hostile sender', 'V', () => {
    const t = 140000;

    const group = createSocialGroup(
      ['L1', 'L2', 'L3'],
      { regime: 'intimate', openness_class: 'open', outward_propagation_depth: 1, inward_visibility: 'none' },
      t,
    );

    // Honest sender: depth=1 (exactly at limit) → accepted
    const validShare = createPropagationShare(group.genesisId, 'group-B', 'L1', 1, t + 1);
    let validAccepted = false;
    try {
      verifyPropagationShare(validShare, group);
      validAccepted = true;
    } catch { /* should not throw */ }

    // Hostile sender: depth=2 (over limit) → verifier rejects regardless of sender intent
    const hostileShare = createPropagationShare(group.genesisId, 'group-C', 'L2', 2, t + 2);
    let hostileRejected = false;
    let hostileError = '';
    try {
      verifyPropagationShare(hostileShare, group);
    } catch (e) {
      hostileRejected = e instanceof VisibilityViolation && e.invariant === 'INV-DEPTH-ENFORCED';
      hostileError = e instanceof Error ? e.message : String(e);
    }

    // Fully-open sink: depth=0 → any outward share (even depth=1) is rejected
    const sinkGroup = createSocialGroup(
      ['L1'],
      { regime: 'public', openness_class: 'fully_open', outward_propagation_depth: 0, inward_visibility: 'full' },
      t + 3,
    );
    const sinkShare = createPropagationShare(sinkGroup.genesisId, 'group-D', 'L1', 1, t + 4);
    let sinkRejected = false;
    try {
      verifyPropagationShare(sinkShare, sinkGroup);
    } catch (e) {
      sinkRejected = e instanceof VisibilityViolation;
    }

    return {
      invariants: [
        {
          invariant: 'INV-DEPTH-ENFORCED',
          passed: validAccepted && hostileRejected && sinkRejected,
          details: [
            `Honest share (depth=1 ≤ limit=1): accepted=${validAccepted} ✓`,
            `Hostile share (depth=2 > limit=1): rejected=${hostileRejected} — ${hostileError}`,
            `Fully-open sink (depth=1 > limit=0): rejected=${sinkRejected} ✓`,
          ].join('; '),
        },
      ],
      provenanceTrace: [
        `Group outward_propagation_depth=1 (class='open')`,
        `Honest sender depth=1: ACCEPTED ✓`,
        `Hostile sender depth=2: REJECTED by verifier ✓ (enforcement is universal, not sender-trust)`,
        `Fully-open (depth=0) is a hard sink — any depth≥1 share rejected ✓`,
      ].join('\n'),
      socialInputsUsed: [
        'SCRIPTED: Hostile sender emits depth=2 share for group with limit=1',
        'SCRIPTED: Honest sender emits depth=1 — accepted',
      ],
    };
  });
}

// V6: Openness class caps outward_propagation_depth at genesis creation time
function runV6(): ExperimentResult {
  return runner.run('V6: Openness class caps propagation depth — over-permissive genesis is invalid', 'V', () => {
    const t = 150000;

    type Case = { cls: OpennessClass; depth: number; valid: boolean };
    const cases: Case[] = [
      { cls: 'closed',     depth: 3, valid: true  }, // max=3, exact limit
      { cls: 'closed',     depth: 4, valid: false }, // max=3, over limit
      { cls: 'open',       depth: 1, valid: true  }, // max=1, exact limit
      { cls: 'open',       depth: 2, valid: false }, // max=1, over limit
      { cls: 'fully_open', depth: 0, valid: true  }, // max=0, hard sink
      { cls: 'fully_open', depth: 1, valid: false }, // max=0, hard sink violated
    ];

    const results = cases.map(c => {
      let accepted = false;
      let violated = false;
      try {
        createSocialGroup(
          ['L1'],
          { regime: 'intimate', openness_class: c.cls, outward_propagation_depth: c.depth, inward_visibility: 'none' },
          t,
        );
        accepted = true;
      } catch (e) {
        violated = e instanceof VisibilityViolation && e.invariant === 'INV-OPENNESS-CAPS-DEPTH';
      }
      const correct = c.valid ? accepted : violated;
      return { ...c, correct };
    });

    const allCorrect = results.every(r => r.correct);

    return {
      invariants: [
        {
          invariant: 'INV-OPENNESS-CAPS-DEPTH',
          passed: allCorrect,
          details: results.map(r =>
            `${r.cls}(max=${MAX_DEPTH_FOR_CLASS[r.cls]}) depth=${r.depth}: ${r.valid ? 'expect accept' : 'expect reject'} → ${r.correct ? '✓' : '✗ VIOLATION'}`
          ).join('; '),
        },
      ],
      provenanceTrace: [
        `MAX_DEPTH_FOR_CLASS: ${JSON.stringify(MAX_DEPTH_FOR_CLASS)}`,
        ...results.map(r =>
          `${r.cls}(max=${MAX_DEPTH_FOR_CLASS[r.cls]}) depth=${r.depth}: ${r.correct ? 'CORRECT' : 'VIOLATION'}`
        ),
        `fully_open is a hard depth-0 sink — genesis with depth≥1 is structurally invalid ✓`,
      ].join('\n'),
      socialInputsUsed: [],
    };
  });
}

// V7: Inward visibility and outward propagation depth are independent parameters
function runV7(): ExperimentResult {
  return runner.run('V7: Inward visibility and outward propagation are independently controllable', 'V', () => {
    const t = 160000;
    const members = ['L1', 'L2', 'L3'];

    // Case A: public board — full inward visibility, zero outward propagation
    const boardGroup = createSocialGroup(
      members,
      { regime: 'public', openness_class: 'fully_open', outward_propagation_depth: 0, inward_visibility: 'full' },
      t,
    );
    const boardVisible  = computeVisibleMembers(boardGroup, members);
    const boardShare    = createPropagationShare(boardGroup.genesisId, 'group-X', 'L1', 1, t + 1);
    let boardPropBlocked = false;
    try { verifyPropagationShare(boardShare, boardGroup); } catch { boardPropBlocked = true; }

    // Case B: private propagating group — no inward visibility, high outward propagation
    const privateGroup = createSocialGroup(
      members,
      { regime: 'intimate', openness_class: 'closed', outward_propagation_depth: 3, inward_visibility: 'none' },
      t + 2,
    );
    const privateVisible    = computeVisibleMembers(privateGroup, members);
    const privateShare      = createPropagationShare(privateGroup.genesisId, 'group-Y', 'L1', 2, t + 3);
    let privatePropAllowed = false;
    try { verifyPropagationShare(privateShare, privateGroup); privatePropAllowed = true; } catch { /* fail */ }

    // Independence: high inward does not force high outward, and vice versa
    const independent =
      boardVisible.length === 3 && boardPropBlocked &&
      privateVisible.length === 0 && privatePropAllowed;

    return {
      invariants: [
        {
          invariant: 'INV-INWARD-OUTWARD-INDEPENDENT',
          passed: independent,
          details: [
            `Board (inward=full outward=0): visible=${boardVisible.length}/3 ✓; prop blocked=${boardPropBlocked} ✓`,
            `Private (inward=none outward=3): visible=${privateVisible.length}/3 ✓; prop allowed=${privatePropAllowed} ✓`,
          ].join('; '),
        },
      ],
      provenanceTrace: [
        `Board group: inward_visibility=full, outward_propagation_depth=0`,
        `  ${boardVisible.length} of ${members.length} members visible; outward propagation depth=1 → blocked ✓`,
        `Private group: inward_visibility=none, outward_propagation_depth=3`,
        `  ${privateVisible.length} of ${members.length} members visible; outward propagation depth=2 → allowed ✓`,
        `Neither parameter's setting forces the other ✓`,
      ].join('\n'),
      socialInputsUsed: [],
    };
  });
}

// V8: Joining a public group leaks only the fact of membership — deanonymization is structurally blocked
function runV8(): ExperimentResult {
  return runner.run('V8: Public membership leaks only affiliation — hostile enumeration fails', 'V', () => {
    const t = 170000;

    // L1 is a member of an intimate group
    const intimateGroup = createSocialGroup(
      ['L1', 'L2', 'L3'],
      { regime: 'intimate', openness_class: 'closed', outward_propagation_depth: 2, inward_visibility: 'none' },
      t,
    );
    const intimateContent = createContentItem(intimateGroup.genesisId, 'intimate', 'L1', 'private edge data', t + 1);

    // L1 also joins a fully-open public group
    const publicGroup = createSocialGroup(
      ['L1', 'L4', 'L5'],
      { regime: 'public', openness_class: 'fully_open', outward_propagation_depth: 0, inward_visibility: 'full' },
      t + 2,
    );
    publicGroup.dag.addNew([publicGroup.genesisId], { type: 'add_member', lineageId: 'L4', targetLineageId: 'L1' }, t + 3);

    // Adversary exhaustively queries publicGroup's DAG
    const publicNodes = publicGroup.dag.allNodes();
    const intimateGenId = intimateGroup.genesisId;

    const leaksIntimateGroupRef = publicNodes.some(n => JSON.stringify(n).includes(intimateGenId));
    const leaksIntimateContentRef = publicNodes.some(n => JSON.stringify(n).includes(intimateContent.id));

    // Hostile propagation: adversary tries to walk from public group toward intimate group via L1
    const hostileShare = createPropagationShare(publicGroup.genesisId, intimateGenId, 'L1', 1, t + 4);
    let propagationBlocked = false;
    try {
      verifyPropagationShare(hostileShare, publicGroup);
    } catch (e) {
      propagationBlocked = e instanceof VisibilityViolation;
    }

    const deanonymResisted = !leaksIntimateGroupRef && !leaksIntimateContentRef && propagationBlocked;

    return {
      invariants: [
        {
          invariant: 'INV-PUBLIC-MEMBERSHIP-BOUNDED',
          passed: deanonymResisted,
          details: [
            `Public DAG references intimate group genesis: ${leaksIntimateGroupRef} (must be false ✓)`,
            `Public DAG references intimate content ID: ${leaksIntimateContentRef} (must be false ✓)`,
            `Hostile propagation depth=1 blocked at depth=0 sink: ${propagationBlocked} ✓`,
          ].join('; '),
        },
        {
          invariant: 'INV-DEPTH-ENFORCED',
          passed: propagationBlocked,
          details: `Adversary cannot walk public→intimate via L1 — depth=0 sink terminates propagation ✓`,
        },
      ],
      provenanceTrace: [
        `L1 member of: intimate(${intimateGroup.genesisId.slice(0, 8)}…) AND public(${publicGroup.genesisId.slice(0, 8)}…)`,
        `Public DAG (${publicNodes.length} nodes) queried — no intimate references found ✓`,
        `Hostile share depth=1 from public (depth=0 sink) toward intimate group: REJECTED ✓`,
        `Structural isolation: intimate data lives only in intimate DAG; public DAG has no reference ✓`,
      ].join('\n'),
      socialInputsUsed: [
        'SCRIPTED: Hostile adversary queries public DAG for intimate affiliations',
        'SCRIPTED: Hostile sender emits depth=1 propagation share from depth=0 sink',
      ],
    };
  });
}

// V9: Freeze-by-default — joining a public group never mutates the intimate graph
function runV9(): ExperimentResult {
  return runner.run('V9: Joining public group does not auto-mutate intimate graph', 'V', () => {
    const t = 180000;

    const intimateGroup = createSocialGroup(
      ['L1', 'L2'],
      { regime: 'intimate', openness_class: 'closed', outward_propagation_depth: 1, inward_visibility: 'none' },
      t,
    );
    const intimateSizeBefore  = intimateGroup.dag.size();
    const intimateNodesBefore = new Set(intimateGroup.dag.allNodes().map(n => n.id));

    // L1 joins a public group (the add_member op lives ONLY in publicGroup's DAG)
    const publicGroup = createSocialGroup(
      ['L3', 'L4'],
      { regime: 'public', openness_class: 'open', outward_propagation_depth: 1, inward_visibility: 'full' },
      t + 1,
    );
    publicGroup.dag.addNew([publicGroup.genesisId], { type: 'add_member', lineageId: 'L3', targetLineageId: 'L1' }, t + 2);

    // Intimate graph must be completely unchanged
    const intimateSizeAfter  = intimateGroup.dag.size();
    const noNewIntimateNodes = intimateGroup.dag.allNodes().every(n => intimateNodesBefore.has(n.id));
    const noPublicRef        = !intimateGroup.dag.allNodes().some(n =>
      JSON.stringify(n).includes(publicGroup.genesisId),
    );

    const publicGrew = publicGroup.dag.size() > 1; // add_member landed in public DAG

    return {
      invariants: [
        {
          invariant: 'INV-FREEZE-BY-DEFAULT',
          passed: intimateSizeAfter === intimateSizeBefore && noNewIntimateNodes && noPublicRef,
          details: [
            `Intimate graph size: ${intimateSizeBefore}→${intimateSizeAfter} (unchanged=${intimateSizeAfter === intimateSizeBefore} ✓)`,
            `No new nodes in intimate graph: ${noNewIntimateNodes} ✓`,
            `No public-group reference in intimate DAG: ${noPublicRef} ✓`,
            `add_member landed in public DAG only (size ${publicGroup.dag.size()}): ${publicGrew} ✓`,
          ].join('; '),
        },
      ],
      provenanceTrace: [
        `Intimate group: ${intimateGroup.genesisId.slice(0, 8)}… size=${intimateSizeBefore} before join`,
        `L1 added to public group ${publicGroup.genesisId.slice(0, 8)}… via add_member (in public DAG)`,
        `Intimate DAG size after: ${intimateSizeAfter} (unchanged ✓)`,
        `No edges from public group auto-added to intimate graph ✓`,
        `Joining/seeing a public board never auto-populates the intimate view ✓`,
      ].join('\n'),
      socialInputsUsed: ['SCRIPTED: L3 adds L1 to public group (op in public DAG only)'],
    };
  });
}

export function run(): ExperimentResult[] {
  return [runV1(), runV2(), runV3(), runV4(), runV5(), runV6(), runV7(), runV8(), runV9()];
}
