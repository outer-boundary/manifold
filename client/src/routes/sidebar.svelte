<script>
	import Icon from "@iconify/svelte";

	let sidebarOpenState = true;
	const closedWidth = "80px";
	const openWidth = "240px";
</script>

<div class="sidebar" style={`width: ${sidebarOpenState ? openWidth : closedWidth}`}>
	<button class="collapseButton" on:click={() => (sidebarOpenState = !sidebarOpenState)}>
		<Icon id="collapseIcon" icon="material-symbols:arrow-back-ios-new-rounded" style={`rotate: ${sidebarOpenState ? '0deg' : '180deg'}`}/>
	</button>
	<p>Sup, I'm the sidebar!</p>
</div>

<style lang="scss">
  $sidebarTransitionTime: 300ms;

	.sidebar {
		outline: 2px solid #313131; // $mainBorderWidth $secondaryElementColour
		border-top-right-radius: 20px; // $mainBorderRadius
		border-bottom-right-radius: 20px; // $mainBorderRadius
		display: flex;
		justify-content: center;
		align-items: center;
		position: relative;
		transition: width $sidebarTransitionTime ease-in-out;
	}

	.collapseButton {
		width: 30px;
		height: 30px;
		position: absolute;
		top: 10px;
		right: 10px;

    #collapseIcon {
			width: 10px;
			height: 10px;
      transition: rotate $sidebarTransitionTime ease-in-out;
		}
	}

  .collapseIcon {
			width: 10px;
			height: 10px;
      transition: rotate $sidebarTransitionTime ease-in-out;
		}

	p {
		color: white; // $mainTextColour
	}
</style>
