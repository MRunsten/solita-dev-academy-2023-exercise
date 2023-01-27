import { PUBLIC_API_ADDRESS } from '$env/static/public'
import { PRIVATE_API_ADDRESS } from '$env/static/private';

import type { HandleFetch } from '@sveltejs/kit';
 
// This handleFetch hook runs on the server and fixes the SSR compile step when running
// through docker-compose. The docker-compose frontend-service cannot access for example
// a localhost URL, as localhost is different inside the frontend-service container.
//
// PRIVATE_API_ADDRESS is a service-specific environment variable defined within 
// docker-compose configuration.
//
// Note that during development, the vite server can often access the "PUBLIC_API_ADDRESS",
// so SSR also works in that case through PUBLIC_API_ADDRESS
export const handleFetch = (async ({request, fetch}) => {
  if (request.url.startsWith(PUBLIC_API_ADDRESS) && PRIVATE_API_ADDRESS !== "") {
    // clone the original request, but change the URL
    request = new Request(
        request.url.replace(PUBLIC_API_ADDRESS, PRIVATE_API_ADDRESS),
        request
    );
  }

  return await fetch(request);
}) satisfies HandleFetch;
