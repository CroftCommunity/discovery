---
name: alt-drive-author-identity
description: "For alt.drive commits the user's git identity is \"Chase Pettet <chase@owasp.org>\" — not chase.mp@owasp.org (stale ref) and not the L360 cpettet@croftc.com identity."
metadata: 
  node_type: memory
  type: reference
  originSessionId: c6c6d6d6-dd3d-4f80-8013-ac8831bc9d05
---

When committing in `/home/ubuntu/alt.drive`, override per-command (CLAUDE.md says never modify git config):

```
git -c user.email="chase@owasp.org" -c user.name="Chase Pettet" commit -m "..."
```

Why the warning: the box's default git identity may be `cpettet@croftc.com` (L360 work email); without the override commits land under the wrong identity.

`VALIDATION.md` once referenced `chase.mp@owasp.org` — that was wrong and was fixed in commit `b0ab7ae`. Don't reintroduce it.

Push uses SSH key at `~/.ssh/id_secroute` (chmod 600). Remote: `git@github.com:AltID/alt.drive.git`. If the key isn't picked up automatically: `GIT_SSH_COMMAND="ssh -i ~/.ssh/id_secroute -o IdentitiesOnly=yes" git push origin main`.

Per [[alt-drive-project]] CLAUDE.md: user makes commits — wait for explicit approval before running `git commit`.
