#!/bin/bash
set -e

echo "==> Building with Trunk..."
trunk build --release

echo "==> Preparing Vercel Build Output..."
rm -rf .vercel/output
mkdir -p .vercel/output/static
mkdir -p .vercel/output/functions/_middleware.func

# Copy static files
cp -r dist/* .vercel/output/static/

# Copy public directory (LP page, etc.)
if [ -d "public" ]; then
  cp -r public/* .vercel/output/static/
fi

# Create Vercel config with middleware route
cat > .vercel/output/config.json << 'EOF'
{
  "version": 3,
  "routes": [
    { "src": "/(.*)", "middlewarePath": "_middleware", "continue": true },
    { "handle": "filesystem" },
    { "src": "/(.*\\.wasm)", "headers": { "Content-Type": "application/wasm", "Cache-Control": "public, max-age=31536000, immutable" }, "continue": true },
    { "src": "/lp", "dest": "/lp.html" },
    { "src": "/(.*)", "dest": "/index.html" }
  ]
}
EOF

# Create Edge Middleware for Basic Auth using ENV VARS
# Set BASIC_AUTH_USER and BASIC_AUTH_PASS in Vercel Environment Variables
cat > .vercel/output/functions/_middleware.func/index.js << 'ENDOFJS'
export default function middleware(request) {
  const expectedUser = process.env.BASIC_AUTH_USER;
  const expectedPass = process.env.BASIC_AUTH_PASS;

  // If no env vars set, skip auth (dev mode)
  if (!expectedUser || !expectedPass) {
    return;
  }

  const auth = request.headers.get("authorization");
  if (auth) {
    const [scheme, encoded] = auth.split(" ");
    if (scheme === "Basic") {
      try {
        const decoded = atob(encoded);
        const [user, pass] = decoded.split(":");
        if (user === expectedUser && pass === expectedPass) {
          return;
        }
      } catch (e) {
        // Invalid base64
      }
    }
  }
  return new Response("Authentication required", {
    status: 401,
    headers: { "WWW-Authenticate": 'Basic realm="FixedAssets"' },
  });
}
ENDOFJS

cat > .vercel/output/functions/_middleware.func/.vc-config.json << 'EOF'
{
  "runtime": "edge",
  "entrypoint": "index.js"
}
EOF

echo "==> Deploying to Vercel..."
vercel deploy --prebuilt --prod

echo "==> Done!"
