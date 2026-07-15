import { createHash } from 'crypto';
import { DAG } from './dag';

export type Regime = 'intimate' | 'public';
export type OpennessClass = 'closed' | 'open' | 'fully_open';
export type InwardVisibility = 'full' | 'partial' | 'none';

// Inverse relationship: openness → maximum allowed outward propagation hops.
// fully_open is a hard depth-0 sink: no affiliation escapes through any member.
export const MAX_DEPTH_FOR_CLASS: Record<OpennessClass, number> = {
  closed:     3,
  open:       1,
  fully_open: 0,
};

export interface SocialGenesisParams {
  regime: Regime;
  openness_class: OpennessClass;
  outward_propagation_depth: number;
  inward_visibility: InwardVisibility;
}

export interface SocialGroup {
  dag: DAG;
  genesisId: string;
  params: SocialGenesisParams;
}

// Content item: regime is part of the signed hash, not client-side metadata.
export interface SocialContentItem {
  id: string;
  groupId: string;
  regime: Regime;
  authorId: string;
  content: string;
  timestamp: number;
}

// Propagation share: a member forwarding their affiliation with a group to another graph node.
export interface PropagationShare {
  id: string;
  fromGroupId: string;
  toGroupId: string;
  memberId: string;
  depth: number;     // hops consumed from the origin group's budget
  timestamp: number;
}

// Republish: crosses regime boundary but carries ONLY sourceHash (reference) + author-chosen publicContent.
export interface RepublishAct {
  id: string;
  targetGroupId: string;
  authorId: string;
  sourceHash: string;    // opaque reference to intimate original; no content embedded
  publicContent: string; // what the author explicitly chose to write
  regime: 'public';      // always public; intimate content does not flow through this op
  timestamp: number;
}

export class VisibilityViolation extends Error {
  constructor(public readonly invariant: string, message: string) {
    super(message);
    this.name = 'VisibilityViolation';
  }
}

// V6 gate: enforced at genesis creation; an over-permissive genesis cannot be constructed.
export function validateGenesisParams(params: SocialGenesisParams): void {
  const maxDepth = MAX_DEPTH_FOR_CLASS[params.openness_class];
  if (params.outward_propagation_depth > maxDepth) {
    throw new VisibilityViolation(
      'INV-OPENNESS-CAPS-DEPTH',
      `outward_propagation_depth=${params.outward_propagation_depth} exceeds max=${maxDepth} for openness_class='${params.openness_class}'`,
    );
  }
}

export function createSocialGroup(
  members: string[],
  params: SocialGenesisParams,
  timestamp: number,
): SocialGroup {
  validateGenesisParams(params);
  const dag = new DAG();
  const genesisNode = dag.addNew(
    [],
    {
      type: 'genesis',
      members,
      regime: params.regime,
      outward_propagation_depth: params.outward_propagation_depth,
      inward_visibility: params.inward_visibility,
      openness_class: params.openness_class,
    },
    timestamp,
  );
  return { dag, genesisId: genesisNode.id, params };
}

// V1: regime is genesis-fixed; this function always throws — the operation is unrepresentable.
export function attemptRegimeChange(_group: SocialGroup, _newRegime: Regime): never {
  throw new VisibilityViolation(
    'INV-REGIME-IMMUTABLE',
    `Cannot change regime — genesis-fixed and hash-linked`,
  );
}

// V2: regime is part of the signed hash for every content item.
export function computeContentId(item: Omit<SocialContentItem, 'id'>): string {
  return createHash('sha256')
    .update(JSON.stringify({
      groupId:   item.groupId,
      regime:    item.regime,    // ← part of signed data
      authorId:  item.authorId,
      content:   item.content,
      timestamp: item.timestamp,
    }))
    .digest('hex');
}

export function createContentItem(
  groupId: string,
  regime: Regime,
  authorId: string,
  content: string,
  timestamp: number,
): SocialContentItem {
  const unsigned = { groupId, regime, authorId, content, timestamp };
  return { id: computeContentId(unsigned), ...unsigned };
}

// V4: republish is a distinct authored act that targets a public group only.
// It carries sourceHash (reference) and author's publicContent — NOT the intimate original's content.
export function createRepublishAct(
  targetGroup: SocialGroup,
  authorId: string,
  sourceItem: SocialContentItem,
  publicContent: string,
  timestamp: number,
): RepublishAct {
  if (targetGroup.params.regime !== 'public') {
    throw new VisibilityViolation(
      'INV-REPUBLISH-DISTINCT',
      `Republish must target a public group; targetGroup regime='${targetGroup.params.regime}'`,
    );
  }
  const fields = {
    targetGroupId: targetGroup.genesisId,
    authorId,
    sourceHash:   sourceItem.id,   // opaque ID; intimate content is NOT embedded
    publicContent,
    regime:       'public' as const,
    timestamp,
  };
  const id = createHash('sha256').update(JSON.stringify(fields)).digest('hex');
  return { id, ...fields };
}

export function createPropagationShare(
  fromGroupId: string,
  toGroupId: string,
  memberId: string,
  depth: number,
  timestamp: number,
): PropagationShare {
  const fields = { fromGroupId, toGroupId, memberId, depth, timestamp };
  const id = createHash('sha256').update(JSON.stringify(fields)).digest('hex');
  return { id, ...fields };
}

// V5: every verifying client runs this check — enforcement does not rely on the sender behaving.
export function verifyPropagationShare(share: PropagationShare, originGroup: SocialGroup): void {
  const maxDepth = originGroup.params.outward_propagation_depth;
  if (share.depth > maxDepth) {
    throw new VisibilityViolation(
      'INV-DEPTH-ENFORCED',
      `Depth ${share.depth} exceeds group's outward_propagation_depth=${maxDepth} — REJECTED by verifier`,
    );
  }
}

// V7: inward_visibility determines member enumeration independent of outward propagation.
export function computeVisibleMembers(group: SocialGroup, members: string[]): string[] {
  switch (group.params.inward_visibility) {
    case 'full':    return [...members];
    case 'partial': return members.slice(0, Math.ceil(members.length / 2));
    case 'none':    return [];
  }
}
