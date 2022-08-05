var parent = chrome.contextMenus.create({
	"id": "PWMPV",
	"title": "Play with MPV",
	"contexts": ["page", "link", "video", "audio"]
});

chrome.contextMenus.onClicked.addListener(function(info, tab) {
	// console.log("item " + info.menuItemId + " was clicked");
	// console.log("info: " + JSON.stringify(info));
	// console.log("tab: " + JSON.stringify(tab));

	let url = (info["linkUrl"] || info["srcUrl"]|| info["pageUrl"]);
    fetch("http://localhost:8080/mpv://"+url)
});