export interface GovernanceDials {
  removalThreshold: number;
  additionThreshold: number;
  quorumSize: number;
  inclusionPriority: 'high' | 'balanced' | 'low';
  genesisFixed: string[];
  runtimeChangeable: string[];
}

export const DIALS_INCLUSION: GovernanceDials = {
  removalThreshold: 0.34,
  additionThreshold: 0.34,
  quorumSize: 2,
  inclusionPriority: 'high',
  genesisFixed: ['removalThreshold'],
  runtimeChangeable: ['quorumSize'],
};

export const DIALS_BALANCED: GovernanceDials = {
  removalThreshold: 0.51,
  additionThreshold: 0.34,
  quorumSize: 3,
  inclusionPriority: 'balanced',
  genesisFixed: ['removalThreshold'],
  runtimeChangeable: ['quorumSize'],
};

export const DIALS_FIDELITY: GovernanceDials = {
  removalThreshold: 0.67,
  additionThreshold: 0.34,
  quorumSize: 4,
  inclusionPriority: 'low',
  genesisFixed: ['removalThreshold'],
  runtimeChangeable: ['quorumSize'],
};

export function meetsRemovalThreshold(
  dials: GovernanceDials,
  votingLineages: number,
  totalLineages: number,
): boolean {
  if (totalLineages === 0) return false;
  return votingLineages / totalLineages >= dials.removalThreshold;
}

export function meetsAdditionThreshold(
  dials: GovernanceDials,
  votingLineages: number,
  totalLineages: number,
): boolean {
  if (totalLineages === 0) return false;
  return votingLineages / totalLineages >= dials.additionThreshold;
}

export function canChangeDial(
  dials: GovernanceDials,
  dialKey: string,
  isGenesis: boolean,
): boolean {
  if (isGenesis) return true;
  return dials.runtimeChangeable.includes(dialKey);
}
