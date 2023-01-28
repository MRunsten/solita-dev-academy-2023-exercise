<script lang="ts">
	import { goto } from '$app/navigation';
	import type { PageData } from '../$types';

	import Pagination from '../../../lib/pagination.svelte';

	function route_user(station_id: string) {
		goto('/station/' + station_id);
	}

	export let data: PageData;

	let table_top: HTMLElement;
	let table_top_mobile: HTMLElement;

	function get_back_to_element() {
		return table_top_mobile ?? table_top;
	}
</script>

<div id="content" bind:this={table_top_mobile}>
	<div class="large-display">
		<Pagination
			sub_url="stations"
			current_page={data.page}
			data_amount={data.stations.length}
			data_max_per_page={data.max_per_page}
		/>
	</div>

	<table class="data-list mobile">
		<thead>
			<tr>
				<td>#</td>
				<td>Capacity</td>
				<td>Name</td>
			</tr>
		</thead>

		<tbody class="mobile">
			{#each data.stations as station}
				<tr on:click={() => goto('/station/' + station.station_id)}>
					<td width="20%"># {station.station_id}</td>
					<td width="20%">{station.capacity}<i class="emoji-on-right">🚲</i></td>
					<td width="60%">
						<table>
							<tr><td><i class="emoji-on-left">🇫🇮</i>{station.name.finnish}</td></tr>
							<tr><td><i class="emoji-on-left">🇸🇪</i>{station.name.swedish}</td></tr>
							<tr><td><i class="emoji-on-left">🇬🇧</i>{station.name.english}</td></tr>
						</table>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>

	<table class="data-list large-display" bind:this={table_top}>
		<thead>
			<tr>
				<td>#</td>
				<td>Capacity</td>
				<td>Name</td>
				<td>City/Operarator</td>
				<td />
			</tr>
		</thead>
		<tbody class="large-display">
			{#each data.stations as station}
				<tr>
					<td width="10%"># {station.station_id}</td>
					<td width="10%">{station.capacity}<i class="emoji-on-right">🚲</i></td>
					<td width="40%">
						<table class="transparent-bg">
							<tr><td><i class="emoji-on-left">🇫🇮</i>{station.name.finnish}</td></tr>
							<tr><td><i class="emoji-on-left">🇸🇪</i>{station.name.swedish}</td></tr>
							<tr><td><i class="emoji-on-left">🇬🇧</i>{station.name.english}</td></tr>
						</table>
					</td>
					<td width="40%">
						<table class="transparent-bg">
							<tr><td>{station.city_name.finnish}</td></tr>
							<tr><td>{station.city_name.swedish}</td></tr>
							<tr><td>{station.operator_name}</td> </tr>
						</table>
					</td>
					<td width="10%">
						<a class="underline-on-hover" style="padding:16px" href="/station/{station.station_id}"
							>View</a
						>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>

	<Pagination
		sub_url="stations"
		current_page={data.page}
		data_amount={data.stations.length}
		data_max_per_page={data.max_per_page}
	/>
</div>

<style lang="scss">
	@import '../../../styles/table.scss';
	@import '../../../styles/emoji.scss';

	#content {
		width: 100%;
		display: flex;
		flex-direction: column;
		padding: 0 16px;
	}

	table.data-list > tbody > tr:nth-child(2n) > td {
		background: $content-background-alt;
	}

	@media only screen and (max-width: 880px) {
		#content {
			padding: 0;
		}
	}
</style>
