var platforms = ["unknown", "win64", "unix"];
var platform_override = null;

function detect_platform() {
    "use strict";

    if (platform_override !== null) {
        return platforms[platform_override];
    }

    var os = "unknown";

    if (navigator.platform == "Linux x86_64") {os = "unix";}
    if (navigator.platform == "Linux i686") {os = "unix";}
    if (navigator.platform == "Linux i686 on x86_64") {os = "unix";}
    if (navigator.platform == "Linux aarch64") {os = "unix";}
    if (navigator.platform == "Linux armv6l") {os = "unix";}
    if (navigator.platform == "Linux armv7l") {os = "unix";}
    if (navigator.platform == "Linux armv8l") {os = "unix";}
    if (navigator.platform == "Linux ppc64") {os = "unix";}
    if (navigator.platform == "Linux mips") {os = "unix";}
    if (navigator.platform == "Linux mips64") {os = "unix";}
    if (navigator.platform == "Mac") {os = "unix";}
    // if (navigator.platform == "Win32") {os = "win32";}
    if (navigator.platform == "Win64" ||
        navigator.userAgent.indexOf("WOW64") != -1 ||
        navigator.userAgent.indexOf("Win64") != -1) { os = "win64"; }
    if (navigator.platform == "FreeBSD x86_64") {os = "unix";}
    if (navigator.platform == "FreeBSD amd64") {os = "unix";}
    if (navigator.platform == "NetBSD x86_64") {os = "unix";}
    if (navigator.platform == "NetBSD amd64") {os = "unix";}

    // I wish I knew by now, but I don't. Try harder.
    if (os == "unknown") {
        // if (navigator.appVersion.indexOf("Win")!=-1) {os = "win32";}
        if (navigator.appVersion.indexOf("Mac")!=-1) {os = "unix";}
        // rust-www/#692 - FreeBSD epiphany!
        if (navigator.appVersion.indexOf("FreeBSD")!=-1) {os = "unix";}
    }

    // Firefox Quantum likes to hide platform and appVersion but oscpu works
    if (navigator.oscpu) {
        // if (navigator.oscpu.indexOf("Win32")!=-1) {os = "win32";}
        if (navigator.oscpu.indexOf("Win64")!=-1) {os = "win64";}
        if (navigator.oscpu.indexOf("Mac")!=-1) {os = "unix";}
        if (navigator.oscpu.indexOf("Linux")!=-1) {os = "unix";}
        if (navigator.oscpu.indexOf("FreeBSD")!=-1) {os = "unix";}
        if (navigator.oscpu.indexOf("NetBSD")!=-1) {os = "unix";}
    }

    return os;
}

function adjust_for_platform() {
    "use strict";

    var platform = detect_platform();

    platforms.forEach(function (platform_elem) {
        var platform_div = document.getElementById("platform-instructions-" + platform_elem);
        platform_div.style.display = "none";
        if (platform === platform_elem) {
            platform_div.style.display = "block";
        }
    });
}

function fill_in_bug_report_values() {
    var nav_plat = document.getElementById("nav-plat");
    var nav_app = document.getElementById("nav-app");
    nav_plat.textContent = navigator.platform;
    nav_app.textContent = navigator.appVersion;
}

(function () {
    adjust_for_platform();
    fill_in_bug_report_values();
}());
