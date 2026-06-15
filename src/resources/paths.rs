struct Path {
    name: &'static str,
    path: &'static str,
}

pub const CACHE_PATHS: [Path; 9] = [
    // chrome
    Path {
        name: "Google Chrome Cache (Windows)",
        path: r"C:\Users\<user>\AppData\Local\Google\Chrome\User Data\Default\Cache",
    },

    // Path {
    //     name: "Google Chrome Cache (macOS)",
    //     path: "~/Library/Caches/Google/Chrome/Default/Cache",
    // },
    // Path {
    //     name: "Google Chrome Cache (Linux)",
    //     path: "~/.cache/google-chrome/Default/Cache",
    // },

    // fire fox
    Path {
        name: "Firefox Cache (Windows)",
        path: r"C:\Users\<user>\AppData\Local\Mozilla\Firefox\Profiles\<profile>\cache2",
    },
    // Path {
    //     name: "Firefox Cache (macOS)",
    //     path: "~/Library/Caches/Firefox/Profiles/<profile>/cache2",
    // },
    // Path {
    //     name: "Firefox Cache (Linux)",
    //     path: "~/.cache/mozilla/firefox/<profile>/cache2",
    // },


    Path {
        name: "Microsoft Edge Cache (Windows)",
        path: r"C:\Users\<user>\AppData\Local\Microsoft\Edge\User Data\Default\Cache",
    },
    // Path {
    //     name: "Microsoft Edge Cache (macOS)",
    //     path: "~/Library/Caches/Microsoft Edge/Default/Cache",
    // },
    // Path {
    //     name: "Microsoft Edge Cache (Linux)",
    //     path: "~/.cache/microsoft-edge/Default/Cache",
    // },

    // ========== APPLE SAFARI ==========
    // Path {
    //     name: "Safari Cache (macOS)",
    //     path: "~/Library/Caches/com.apple.Safari",
    // },
    // Path {
    //     name: "Safari WebKit Cache (macOS)",
    //     path: "~/Library/Caches/com.apple.WebKit.PluginProcess",
    // },

    // // ========== OPERA ==========
    Path {
        name: "Opera Cache (Windows)",
        path: r"C:\Users\<user>\AppData\Local\Opera Software\Opera Stable\Cache",
    },
    // Path {
    //     name: "Opera Cache (macOS)",
    //     path: "~/Library/Caches/com.operasoftware.Opera",
    // },
    // Path {
    //     name: "Opera Cache (Linux)",
    //     path: "~/.cache/opera",
    // },


    Path {
        name: "Windows System Cache",
        path: r"C:\Windows\Temp",
    },
    Path {
        name: "Windows User Temp",
        path: r"C:\Users\<user>\AppData\Local\Temp",
    },
    // Path {
    //     name: "macOS System Cache",
    //     path: "/Library/Caches",
    // },
    // Path {
    //     name: "macOS User Cache",
    //     path: "~/Library/Caches",
    // },
    // Path {
    //     name: "Linux System Cache",
    //     path: "/var/cache",
    // },
    // Path {
    //     name: "Linux User Cache (XDG)",
    //     path: "~/.cache",
    // },
    // Path {
    //     name: "Linux User Temp",
    //     path: "/tmp",
    // },
// other apps
    Path {
        name: "Discord Cache (Windows)",
        path: r"C:\Users\<user>\AppData\Roaming\discord\Cache",
    },
    Path {
        name: "Spotify Cache (Windows)",
        path: r"C:\Users\<user>\AppData\Local\Spotify\Storage",
    },
    Path {
        name: "VS Code Cache (Windows)",
        path: r"C:\Users\<user>\AppData\Roaming\Code\Cache",
    },
];