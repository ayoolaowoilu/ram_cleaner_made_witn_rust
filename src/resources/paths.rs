pub struct Path {
  pub name: &'static str,
   pub path: &'static str,
}

pub const CACHE_PATHS: [Path; 9] = [
    // chrome
    Path {
        name: "Google Chrome Cache",
        path: r"C:\Users\<user>\AppData\Local\Google\Chrome\User Data\Default\Cache\Cache_Data",
    },
    Path {
        name: "Firefox Cache",
        path: r"C:\Users\<user>\AppData\Local\Mozilla\Firefox\Profiles\<profile>\cache2",
    },
    Path {
        name: "Microsoft Edge Cache",
        path: r"C:\Users\<user>\AppData\Local\Microsoft\Edge\User Data\Default\Cache",
    },

    Path {
        name: "Opera Cache",
        path: r"C:\Users\<user>\AppData\Local\Opera Software\Opera Stable\Cache",
    },
  


    Path {
        name: "Windows System Cache",
        path: r"C:\Windows\Temp",
    },
    Path {
        name: "Windows User Temp",
        path: r"C:\Users\<user>\AppData\Local\Temp",
    },
  
// other apps
    Path {
        name: "Discord Cache",
        path: r"C:\Users\<user>\AppData\Roaming\discord\Cache\Cache_Data",
    },
    Path {
        name: "Spotify Cache",
        path: r"C:\Users\<user>\AppData\Local\Spotify\Storage",
    },
    Path {
        name: "VS Code Cache (Windows)",
        path: r"C:\Users\<user>\AppData\Roaming\Code\Cache\Cache_Data",
    },
];