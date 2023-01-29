<script lang="ts">
	import { page } from '$app/stores';

	import type { LayoutData  } from './$types';
	
	export let data: LayoutData;
</script>

<div id="app">
	<div id="menu">
		<h1>City bicycles</h1>

		<nav>
			<a href="/journeys" class:active={$page.url.pathname.includes('journeys')}>Journeys</a>
			<a href="/stations" class:active={$page.url.pathname.includes('stations')}>Stations</a>
		</nav>
	</div>

	<div id="content">
		<slot />
	</div>
</div>

<style lang="scss">
	@import '../app.scss';

	$app-background: $content-background;
	$menu-background: $color2;
	$menu-background-text: $color2-text;
	$nav-background: $color1;

	$nav-button-text: $color1-text;
	$nav-button-background: $color1;
	$nav-button-hover-background: $color2;
	$nav-button-active-background: $color3;

	#app {
		display: flex;
		background: $app-background;
		width: 100vw;
		height: 100vh;
		overflow: hidden;
		font-family: 'Roboto', sans-serif;
	}

	#content {
		flex: 7;
		display: flex;
		background: $app-background;
		overflow: auto;
	}

	#menu {
		flex: 1;
		display: flex;
		height: 100vh;
		flex-basis: 150px;
		box-sizing: border-box;
		flex-direction: column;
		background: $menu-background;
		box-shadow: 0 0 8px black;
		z-index: 1;
	}

	h1 {
		white-space: nowrap;
		margin: 0;
		padding: 16px 24px;
		color: $menu-background-text;
		font-size: 1.5rem;
		display: flex;
	}

	nav {
        height:100%;
		padding-top: 12px;
		display: flex;
		flex-direction: column;
		background: $nav-background;
	}

	nav a {
		background: $nav-button-background;
		padding: 20px 20px;
		font-size: 1.2rem;

		margin: 8px 16px;
		border-radius:8px;

		color: $nav-button-text;
		text-decoration: none;

		transition:all 0.1s;
	}

	nav a:hover {
		background: $nav-button-hover-background;
	}

	nav a.active {
		background: $nav-button-active-background;
		border-right: 4px solid $accent;
	}

	nav a:hover,
	nav a.active:hover {
		background: $nav-button-hover-background;
	}

	@media only screen and (max-width: 880px) {
		#app {
			flex-direction: column;
		}

		h1 {
			font-size: 1.2rem;
		}

		#menu {
			flex: 0;
			flex-direction: row;
			height: 56px;
		}

		nav {
			background: $nav-background;
			padding-top: 0;
			flex-direction: row;
			flex-grow: 1;
		}

		nav a {
			height: 56px;
			padding: 0;
			margin:0;
			border-radius: 0;
			flex-grow: 1;
			text-align: center;
			line-height: 52px;
			box-sizing: border-box;
			font-size: 1rem;
		}

		nav a {
			background: $menu-background;
		}

		nav a.active {
			background: $menu-background;
			border-bottom: 4px solid $accent;
			border-right: 0;
		}
	}
</style>
