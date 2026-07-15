export interface SocialDecision {
  type: 'follow_branch' | 're_formation' | 'accept_checkpoint';
  decidedBy: string;
  chosenBranchHead: string;
  rejectedBranchHead?: string;
  reason: string;
}

export function scriptFollowBranch(
  decidedBy: string,
  chosenBranchHead: string,
  rejectedBranchHead: string,
  reason: string,
): SocialDecision {
  return { type: 'follow_branch', decidedBy, chosenBranchHead, rejectedBranchHead, reason };
}

export function scriptReFormation(
  decidedBy: string,
  newGroupHead: string,
  reason: string,
): SocialDecision {
  return { type: 're_formation', decidedBy, chosenBranchHead: newGroupHead, reason };
}
