// Vercel Edge Middleware: Inject geo-IP country code into HTML as meta tag
// The WASM client reads this to auto-detect the visitor's country
export default async function middleware(request) {
  const country = request.geo?.country || '';

  // Fetch the original response
  const response = await fetch(request);

  // Only modify HTML responses
  const contentType = response.headers.get('content-type') || '';
  if (!contentType.includes('text/html')) {
    return response;
  }

  // Read the HTML body
  const html = await response.text();

  // Inject country meta tag into <head>
  const injectedHtml = html.replace(
    '</head>',
    `<meta name="x-vercel-ip-country" content="${country}" />\n</head>`
  );

  return new Response(injectedHtml, {
    status: response.status,
    headers: {
      ...Object.fromEntries(response.headers.entries()),
      'content-type': 'text/html; charset=utf-8',
    },
  });
}

export const config = {
  matcher: ['/', '/welcome'],
};
