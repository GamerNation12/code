const key = "$2a$10$V8z1wndBlA7Q82DGkdJ3y.yFGFTQ/Ggg0fMsSYbRVSs.zJBGPNK6y";
fetch('https://api.curseforge.com/v1/mods/search?gameId=432&classId=4471&sortField=1&sortOrder=desc', {
    headers: {
        'x-api-key': key,
        'User-Agent': 'PrismLauncher/8.0',
        'Accept': 'application/json'
    }
}).then(res => {
    console.log(res.status);
    return res.text();
}).then(console.log).catch(console.error);
