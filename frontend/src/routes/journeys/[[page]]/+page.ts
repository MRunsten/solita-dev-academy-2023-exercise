import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load = (({ params }) => {
    return {
        page: params.page || "0",
    };

    throw error(404, 'Not found');
}) satisfies PageLoad;
