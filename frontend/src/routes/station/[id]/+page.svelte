<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div id="content">
	<table class="data-list mobile-no-margin">
		<thead>
			<tr><td><h2 style="margin-left:16px">Bicycle Station {data.station_id}</h2></td></tr>
		</thead>

		<tbody>
			<tr><td><i class="emoji-on-left">🇫🇮</i>{data.name.finnish}</td></tr>
			<tr><td><i class="emoji-on-left">🇸🇪</i>{data.name.swedish}</td></tr>
			<tr><td><i class="emoji-on-left">🇬🇧</i>{data.name.english}</td></tr>
		</tbody>
	</table>

	<table class="data-list">
		<thead>
			<tr>
				<td width="50%"><span>Address</span></td>
				<td width="50%"><span>City</span></td>
			</tr>
		</thead>
		<tbody>
			<tr>
				<td><i class="emoji-on-left">🇫🇮</i>{data.address.finnish}</td>
				{#if data.city.finnish !== ' '}
					<td>{data.city.finnish}</td>
				{:else}
					<td>No Finnish city name available.</td>
				{/if}
			</tr>
			<tr>
				<td><i class="emoji-on-left">🇸🇪</i>{data.address.swedish}</td>
				{#if data.city.swedish !== ' '}
					<td>{data.city.swedish}</td>
				{:else}
					<td>No Swedish city name available.</td>
				{/if}
			</tr>
		</tbody>
	</table>

	<table class="data-list mobile-no-margin">
		<thead>
			<tr><td><span>Operator</td></tr>
		</thead>

		<tbody>
			<tr>
				{#if data.operator_name !== ' '}
					<td>{data.operator_name}</td>
				{:else}
					<td>No operator information available.</td>
				{/if}
			</tr>
		</tbody>
	</table>

	<table class="data-list">
		<thead>
			<tr>
				<td width="75%"><span>Data</span></td>
				<td><span>Value</span></td>
			</tr>
		</thead>
		<tbody>
			<tr>
				<td>Total ending journeys to this station</td>
				<td>{data.total_ending_journeys}</td>
			</tr>

			<tr>
				<td>Total starting journeys from this station</td>
				<td>{data.total_starting_journeys}</td>
			</tr>
		</tbody>
	</table>
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

	table.data-list {
		margin-top: 16px;
	}

	table.data-list > tbody > tr:nth-child(2n) > td {
		background: $content-background-alt;
	}

	@media only screen and (max-width: 880px) {
		h2 {
			margin: 0;
		}

		#content {
			padding: 0;
		}

		table.data-list,
		table.mobile-no-margin {
			margin-top: 0;
		}

		table.data-list > thead > tr > td {
			padding:0;
		}

		table.data-list > tbody > tr > td {
			padding-left:16px;
		}
	}
</style>
