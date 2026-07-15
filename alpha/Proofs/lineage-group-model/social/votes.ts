export interface Vote {
  lineageId: string;
  targetLineageId: string;
  action: 'add' | 'remove' | 'keep';
  reason: string;
}

export interface VoteResult {
  action: 'add' | 'remove' | 'no_quorum';
  forVotes: number;
  totalLineages: number;
  quorumMet: boolean;
}

export function scriptedVote(votes: Vote[]): Map<string, Vote[]> {
  const result = new Map<string, Vote[]>();
  for (const vote of votes) {
    if (!result.has(vote.targetLineageId)) result.set(vote.targetLineageId, []);
    result.get(vote.targetLineageId)!.push(vote);
  }
  return result;
}

export function tallyVotes(
  votes: Vote[],
  targetAction: 'add' | 'remove',
  threshold: number,
  totalLineages: number,
): VoteResult {
  const forVotes = votes.filter(v => v.action === targetAction).length;
  const quorumMet = totalLineages > 0 && forVotes / totalLineages >= threshold;
  return {
    action: quorumMet ? targetAction : 'no_quorum',
    forVotes,
    totalLineages,
    quorumMet,
  };
}
