<script>
	import DomainCard from "./DomainCard.svelte";

	const wallpapers = [
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwallup.net%2Fwp-content%2Fuploads%2F2016%2F03%2F10%2F343202-landscape-nature.jpg&f=1&nofb=1&ipt=168a2794fc43075e38ccc26608bdb8aaa2e068f2691aa49a3686ca0b34493134&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwallup.net%2Fwp-content%2Fuploads%2F2016%2F03%2F10%2F343202-landscape-nature.jpg&f=1&nofb=1&ipt=168a2794fc43075e38ccc26608bdb8aaa2e068f2691aa49a3686ca0b34493134&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi2.wp.com%2Ftechbeasts.com%2Fwp-content%2Fuploads%2F2016%2F01%2Fgreen_mountain_nature_wallpaper_hd.jpg&f=1&nofb=1&ipt=caba1f81356ad66a469eb12d441fca5acf7699d7471ef5203d193deb79aa9b0a&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2F1.bp.blogspot.com%2F-up45dJ1vwEE%2FUMAw3fWxviI%2FAAAAAAAAOkE%2FAKrDgMDkLI0%2Fs1600%2FGreen%2BNature%2BWallpapers.jpg&f=1&nofb=1&ipt=dcf60b1cddeaf2e6d056c3b5a93fb60cc22d194ce410dd45414fb2d354c8f7f6&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%3Fid%3DOIP.ithVPh1_xajTlhHyoZmHOgHaEo%26pid%3DApi&f=1&ipt=12da6540536c488e079714fa1feee57f558e0cadc0dc649f47d4a471572ffd03&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%3Fid%3DOIP.DwTdZwLSYEiXy5EscPrhWQHaEK%26pid%3DApi&f=1&ipt=9e6cd5d14caba0cc6fbd43b22119f02736a79a673ccf906b6ac95e161b49e6ef&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse2.mm.bing.net%2Fth%3Fid%3DOIP.yiXkBeXVDsCRL6KqQtNWAwHaEK%26pid%3DApi&f=1&ipt=2629af6ad52b3fe14209edd785a3c13b7bbcc5a5268161504bf9434b8152f177&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fs1.picswalls.com%2Fwallpapers%2F2017%2F12%2F11%2Fawesome-nature-wallpaper_123025727_313.jpg&f=1&nofb=1&ipt=6a6f72f6a0b80bd8b59ca60c425aeca4c3d05bdbbf88c719535dc6ad4e74f9f7&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse2.mm.bing.net%2Fth%3Fid%3DOIP.eqqnIaxaDi5VypDVyMaUHwHaE7%26pid%3DApi&f=1&ipt=9e71dddcc3cb1444841cec6ea702c2cae7092cfc758031594f8e9c0fead4f765&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fgetwallpapers.com%2Fwallpaper%2Ffull%2F9%2F8%2F0%2F291181.jpg&f=1&nofb=1&ipt=4620af84d37cb49178d75434858ee780d741fda32f416154c282c27920c059bd&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.techgrapple.com%2Fwp-content%2Fuploads%2F2016%2F08%2Fnature-green-wallpaper-hd.jpg&f=1&nofb=1&ipt=0f8cc27cc6262bf9a4c9691f6addc3d1776f5ede8197c121f97ac8d0af80fc35&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%3Fid%3DOIP.vJH0drO5V3iaMYeG5YWbFAHaEK%26pid%3DApi&f=1&ipt=725f791eb8e1cb8d6b37cfcf5b9dc92d953f0d3080a394e1d6d48c2df18ccd42&ipo=images",
		"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.pixelstalk.net%2Fwp-content%2Fuploads%2F2016%2F06%2FHD-images-of-nature-download.jpg&f=1&nofb=1&ipt=75a9c039abf5e3a1f378a14e104ad8a102c021d919cd60170104e03d11fedded&ipo=images"
	];
</script>

<div class="domains">
	<div class="favouriteDomainsContainer">
		<p class="title">Favourites</p>
		<div class="favouriteDomains">
			{#each [...new Array(10)] as card}
				<DomainCard
					cardType="favourite"
					name="My Cool Domain"
					memberCount={Math.ceil(Math.random() * 50)}
					wallpaperUrl={wallpapers[Math.floor(Math.random() * wallpapers.length)]}
				/>
			{/each}
		</div>
	</div>

	<div class="allDomainsContainer">
		<p class="title">All</p>
		<div class="allDomains">
			{#each [...new Array(12)] as card}
				<DomainCard
					cardType="all"
					name="My Cool Domain"
					memberCount={Math.ceil(Math.random() * 50)}
					wallpaperUrl={wallpapers[Math.floor(Math.random() * wallpapers.length)]}
				/>
			{/each}
		</div>
	</div>
</div>

<style lang="scss">
	@import "../../styles/globalStyles.scss";

	.domains {
		width: 100%;
		height: 100%;
		padding: 100px 100px;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		overflow: hidden;
	}

	.favouriteDomainsContainer {
		width: 100%;
		height: 40%;
		display: flex;
		flex-direction: column;
		margin-bottom: 60px;
	}

	.title {
		color: $mainTextColour;
		margin-bottom: 20px;
		font-size: 1.2rem;
	}

	.favouriteDomains {
		width: 100%;
		height: 100%;
		display: flex;
		overflow-x: auto;
		gap: 40px;

		& :global(.card) {
			min-width: 30%;
			height: 100%;
		}
	}

	.allDomainsContainer {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.allDomains {
		width: 100%;
		height: 100%;
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		grid-auto-rows: 65%;
		overflow-y: auto;
		gap: 40px 60px;

		& > :global(.card) {
			width: 100%;
			height: 100%;
		}
	}
</style>
