#!/usr/bin/env bash
# Assembles three push-ready GitHub Pages repo folders under deploy/ for the authoritative
# K1 run on real *.croft.ing. Each folder is a self-contained repo root: drop its contents
# into a new repo, add the CNAME (already written), enable Pages + Enforce HTTPS.
#
#   deploy/kernel-k1  -> kernel-k1.croft.ing   (serves the kernel iframe at /kernel/)
#   deploy/appa-k1    -> appa-k1.croft.ing     (skin A; KERNEL_ORIGIN baked in)
#   deploy/appb-k1    -> appb-k1.croft.ing     (skin B; identical to A)
#
# Change KERNEL_ORIGIN / the three hostnames here if you want different subdomain names.
set -euo pipefail
cd "$(dirname "$0")"

KERNEL_HOST="kernel-k1.croft.ing"
APPA_HOST="appa-k1.croft.ing"
APPB_HOST="appb-k1.croft.ing"
KERNEL_ORIGIN="https://${KERNEL_HOST}"

rm -rf deploy
mkdir -p "deploy/kernel-k1/kernel" "deploy/appa-k1" "deploy/appb-k1"

# --- kernel repo: keep the /kernel/ path so the iframe src is identical to local ---
cp kernel/index.html kernel/kernel.js "deploy/kernel-k1/kernel/"
printf '%s\n' "$KERNEL_HOST" > "deploy/kernel-k1/CNAME"
cat > "deploy/kernel-k1/index.html" <<'HTML'
<!doctype html><meta charset="utf-8"><title>account-kernel K1</title>
<p>account-kernel K1 kernel origin. The probe iframe is at <a href="kernel/">/kernel/</a>.</p>
HTML

# --- app repos: app page at repo root, KERNEL_ORIGIN injected before app.js ---
build_app () {
  local dir="$1" host="$2"
  cp app/app.js "$dir/app.js"
  perl -pe 's{<script src="app.js"></script>}{<script>window.KERNEL_ORIGIN="'"$KERNEL_ORIGIN"'";</script>\n  <script src="app.js"></script>}' \
    app/index.html > "$dir/index.html"
  printf '%s\n' "$host" > "$dir/CNAME"
}
build_app "deploy/appa-k1" "$APPA_HOST"
build_app "deploy/appb-k1" "$APPB_HOST"

echo "built deploy/ ->"
find deploy -type f | sort
echo
echo "KERNEL_ORIGIN baked into app bundles:"
grep -h "KERNEL_ORIGIN" deploy/appa-k1/index.html
