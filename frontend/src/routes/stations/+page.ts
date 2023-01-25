import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load = (({ params }) => {
    return {

    };

    throw error(404, 'Not found');
}) satisfies PageLoad;
